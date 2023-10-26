use std::{collections::HashMap, str::FromStr};

use derivative::Derivative;
use ethabi::ethereum_types::{H160, U256};
use model::scenario::{
    Action as ActionModel, Scenario as ScenarioModel, Script as ScriptModel,
    Trigger as TriggerModel,
};

use crate::{
    action::{
        data::InputData as ActionInputData,
        storage::{Action, ActionKindType},
        values::get_actions,
        ActionGroup,
    },
    format_address,
    input::{InputType, OffChainInputType},
    source::{
        data::InputData as SourceInputData,
        storage::{Source, SourceKindType},
        values::get_sources,
    },
    ChainId, Valid,
};

pub mod calldata;
pub mod validator;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown name")]
    UnknownName,
    #[error("Unknown chain ID")]
    UnknownChainId,
    #[error("Missing on chain data schema")]
    MissingOnChainSchema,
    #[error("Missing link parameter")]
    MissingLinkParameter,
    #[error("Invalid link parameter")]
    InvalidLinkParameter,
    #[error("Missing actor address")]
    MissingActorAddress,
    #[error("Missing input token")]
    MissingInputToken,
    #[error("Missing input token amount")]
    MissingInputTokenAmount,
    #[error("Missing data field {0}")]
    MissingDataField(String),
    #[error("Parsing data field {0} error")]
    ParsingDataField(String),
    #[error("Invalid input data")]
    InputValidation,
    #[error("No on-chain actions")]
    NoOnChainActions,
}

#[derive(Debug, thiserror::Error)]
pub enum TaggedError {
    #[error("Trigger parser error (source #{source_idx}): {error}")]
    Trigger { source_idx: u32, error: Error },
    #[error("Action parser error (action #{action_idx}, {group} group): {error}")]
    Action {
        action_idx: u32,
        group: ActionGroup,
        error: Error,
    },
    #[error(transparent)]
    Error(Error),
    #[error("Script #{script_idx}: {error}")]
    Scenario {
        script_idx: u32,
        error: Box<TaggedError>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriggerType {
    ALL = 0,
    ANY = 1,
}

impl TryFrom<u8> for TriggerType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => TriggerType::ALL,
            1 => TriggerType::ANY,
            _ => return Err(Error::InputValidation),
        })
    }
}

impl TriggerType {
    fn as_u8(&self) -> u8 {
        match self {
            TriggerType::ALL => 0,
            TriggerType::ANY => 1,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ParsedAction {
    pub data: Valid<ActionInputData>,
    #[derivative(Debug = "ignore")]
    pub action: &'static Action,
    #[derivative(Debug = "ignore")]
    #[allow(clippy::borrowed_box)]
    pub action_kind: &'static Box<ActionKindType>,
    pub input_token: Option<H160>,
    pub output: Option<H160>,
}

impl ParsedAction {
    fn get_model(self) -> ActionModel {
        let Valid(ActionInputData::Raw { data }) = self.data else {
            unreachable!()
        };

        ActionModel {
            action: self.action.name.clone(),
            input_token: self.input_token.map(format_address),
            output: self.output.map(format_address),
            data,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ParsedTrigger {
    pub data: Valid<SourceInputData>,
    #[derivative(Debug = "ignore")]
    pub source: &'static Source,
    #[derivative(Debug = "ignore")]
    #[allow(clippy::borrowed_box)]
    pub source_kind: &'static Box<SourceKindType>,
    pub on_chain_verification: bool,
}

impl ParsedTrigger {
    fn get_model(self) -> TriggerModel {
        let Valid(SourceInputData::Raw { data, condition }) = self.data else {
            unreachable!()
        };

        TriggerModel {
            source: self.source.name.clone(),
            kind: self.source_kind.name().to_owned(),
            data,
            condition,
            on_chain_verification: self.on_chain_verification,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedScript {
    pub trigger_type: TriggerType,
    pub triggers: Vec<ParsedTrigger>,
    pub on_chain_actions: Vec<ParsedAction>,
    pub off_chain_actions: Vec<ParsedAction>,
}

impl ParsedScript {
    pub fn triggers(&self) -> &Vec<ParsedTrigger> {
        &self.triggers
    }

    pub fn get_model(self) -> ScriptModel {
        ScriptModel {
            trigger_type: self.trigger_type.as_u8(),
            triggers: self.triggers.into_iter().map(|t| t.get_model()).collect(),
            on_chain_actions: self
                .on_chain_actions
                .into_iter()
                .map(|a| a.get_model())
                .collect(),
            off_chain_actions: self
                .off_chain_actions
                .into_iter()
                .map(|a| a.get_model())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedScenario {
    pub id: u32,
    pub on_chain_id: Option<u32>,
    pub name: String,
    pub chain_id: ChainId,
    pub owner: H160,
    pub actor_id: Option<u32>,
    pub actor: Option<H160>,
    pub input_token: Option<H160>,
    pub input_amount: Option<U256>,
    pub scripts: Vec<ParsedScript>,
}

pub trait FromModel {
    type Error;
    type Result;

    fn parse(self) -> Result<Self::Result, Self::Error>;
}

impl FromModel for ActionModel {
    type Error = Error;
    type Result = ParsedAction;

    fn parse(self) -> Result<Self::Result, Self::Error> {
        let actions = get_actions();
        let action = actions
            .action_by_name(&self.action)
            .ok_or(Error::UnknownName)?;
        let action_kind = action.get();

        let mut input_token = None;
        if let Some(input_token_value) = self.input_token {
            input_token = Some(
                H160::from_str(&input_token_value)
                    .map_err(|_| Error::ParsingDataField("input_token".to_owned()))?,
            );
        }

        let mut output = None;
        if let Some(output_value) = self.output {
            output = Some(
                H160::from_str(&output_value)
                    .map_err(|_| Error::ParsingDataField("output".to_owned()))?,
            );
        }

        let mut data = HashMap::new();
        for (input_name, input_type) in action_kind.raw_input_schema().data.iter() {
            if !self.data.contains_key(input_name) {
                return Err(Error::MissingDataField(input_name.clone()));
            }

            data.insert(
                input_name.clone(),
                input_type
                    .parse_value(self.data.get(input_name).unwrap())
                    .ok_or(Error::ParsingDataField(input_name.clone()))?
                    .stringify(),
            );
        }

        if let Some(link) = action_kind.raw_input_schema().link {
            let field = self.data.get(link).ok_or(Error::MissingLinkParameter)?;
            let value: String = InputType::OffChain(OffChainInputType::Number(16))
                .parse_value(field)
                .ok_or(Error::InvalidLinkParameter)?
                .stringify();

            data.insert(link.to_owned(), value);
        }

        let valid_data = action_kind
            .validate_input_data(ActionInputData::Raw { data })
            .ok_or(Error::InputValidation)?;

        Ok(ParsedAction {
            data: valid_data,
            action,
            action_kind,
            input_token,
            output,
        })
    }
}

impl FromModel for TriggerModel {
    type Error = Error;
    type Result = ParsedTrigger;

    fn parse(self) -> Result<Self::Result, Self::Error> {
        let sources = get_sources();
        let source = sources
            .source_by_name(&self.source)
            .ok_or(Error::UnknownName)?;
        let source_kind = source.kind_by_name(&self.kind).ok_or(Error::UnknownName)?;

        let mut data = HashMap::new();
        for (input_name, input_type) in source_kind.raw_input_schema().data.iter() {
            if !self.data.contains_key(input_name) {
                return Err(Error::MissingDataField(input_name.clone()));
            }

            data.insert(
                input_name.clone(),
                input_type
                    .parse_value(self.data.get(input_name).unwrap())
                    .ok_or(Error::ParsingDataField(input_name.clone()))?
                    .stringify(),
            );
        }

        if let Some(link) = source_kind.raw_input_schema().link {
            let field = self.data.get(link).ok_or(Error::MissingLinkParameter)?;
            let value = InputType::OffChain(OffChainInputType::Number(16))
                .parse_value(field)
                .ok_or(Error::InvalidLinkParameter)?
                .stringify();

            data.insert(link.to_owned(), value);
        }

        let valid_data = source_kind
            .validate_input_data(SourceInputData::Raw {
                data,
                condition: self.condition,
            })
            .ok_or(Error::InputValidation)?;

        Ok(ParsedTrigger {
            data: valid_data,
            source,
            source_kind,
            on_chain_verification: self.on_chain_verification,
        })
    }
}

impl FromModel for ScriptModel {
    type Error = TaggedError;
    type Result = ParsedScript;

    fn parse(self) -> Result<Self::Result, Self::Error> {
        let mut triggers = Vec::with_capacity(self.triggers.len());
        for (idx, trigger) in self.triggers.into_iter().enumerate() {
            triggers.push(trigger.parse().map_err(|e| TaggedError::Trigger {
                source_idx: idx as u32 + 1,
                error: e,
            })?)
        }

        let mut on_chain_actions = Vec::with_capacity(self.on_chain_actions.len());
        for (idx, on_chain_action) in self.on_chain_actions.into_iter().enumerate() {
            on_chain_actions.push(on_chain_action.parse().map_err(|e| TaggedError::Action {
                action_idx: idx as u32 + 1,
                group: ActionGroup::OnChain,
                error: e,
            })?);
        }

        let mut off_chain_actions = Vec::with_capacity(self.off_chain_actions.len());
        for (idx, off_chain_action) in self.off_chain_actions.into_iter().enumerate() {
            off_chain_actions.push(off_chain_action.parse().map_err(|e| TaggedError::Action {
                action_idx: idx as u32 + 1,
                group: ActionGroup::OffChain,
                error: e,
            })?);
        }

        Ok(ParsedScript {
            trigger_type: self.trigger_type.try_into().map_err(TaggedError::Error)?,
            triggers,
            on_chain_actions,
            off_chain_actions,
        })
    }
}

impl FromModel for ScenarioModel {
    type Error = TaggedError;
    type Result = ParsedScenario;

    fn parse(self) -> Result<Self::Result, Self::Error> {
        let owner = H160::from_str(&self.owner)
            .map_err(|_| TaggedError::Error(Error::ParsingDataField("owner".to_owned())))?;

        let actor = self
            .actor_address
            .map(|v| {
                H160::from_str(&v).map_err(|_| {
                    TaggedError::Error(Error::ParsingDataField("actor_address".to_owned()))
                })
            })
            .transpose()?;

        let input_token = self
            .input_token
            .map(|v| {
                H160::from_str(&v).map_err(|_| {
                    TaggedError::Error(Error::ParsingDataField("input_token".to_owned()))
                })
            })
            .transpose()?;

        let input_amount = self
            .input_amount
            .map(|v| {
                U256::from_dec_str(&v).map_err(|_| {
                    TaggedError::Error(Error::ParsingDataField("input_amount".to_owned()))
                })
            })
            .transpose()?;

        let mut scripts = Vec::with_capacity(self.scripts.len());
        for (idx, script) in self.scripts.0.into_iter().enumerate() {
            scripts.push(script.parse().map_err(|e| TaggedError::Scenario {
                script_idx: idx as u32 + 1,
                error: Box::new(e),
            })?);
        }

        Ok(ParsedScenario {
            id: self.id as u32,
            on_chain_id: self.on_chain_id.as_option(),
            chain_id: self.network,
            name: self.name,
            owner,
            actor_id: self.actor_id.as_option(),
            actor,
            input_token,
            input_amount,
            scripts,
        })
    }
}
