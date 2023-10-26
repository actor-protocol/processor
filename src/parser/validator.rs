use ethabi::ethereum_types::{H160, U256};
use model::scenario::Script;

use super::{ParsedScenario, ParsedScript};
use crate::{
    action::{data::InputData as ActionInputData, ActionGroup},
    input::{InputType, InputValue, OffChainInputType, OffChainInputValue},
    source::data::InputData as SourceInputData,
    ChainId, Valid,
};

#[async_trait::async_trait]
pub trait ValidateParsed {
    type Error;
    type Metadata;
    type Result;

    async fn validate(self, meta: Self::Metadata) -> Result<Self::Result, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid actions sequence")]
    InvalidActionSequence,
    #[error("Invalid action group usage")]
    InvalidActionGroup,
    #[error("Empty triggers list in script")]
    EmptyTriggersList,
    #[error("Empty action list in script")]
    EmptyActionsList,
    #[error("Invalid scripts sequence")]
    InvalidScriptSequence,
}

#[derive(Debug)]
pub struct ValidParsedScenario {
    pub(super) id: u32,
    pub(super) chain_id: ChainId,
    pub(super) owner: H160,
    pub(super) actor: Option<H160>,
    pub(super) input_token: Option<H160>,
    pub(super) input_amount: Option<U256>,
    pub(super) scripts: Vec<Valid<ParsedScript>>,
}

pub type LinkData = (&'static str, u32);
impl ValidParsedScenario {
    pub fn export_links(&self) -> Vec<LinkData> {
        let mut links: Vec<LinkData> = Vec::new();

        for script in self.scripts.iter() {
            for trigger in script.0.triggers.iter() {
                if let Some(link) = trigger.source_kind.raw_input_schema().link {
                    let SourceInputData::Raw { data, condition: _ } = &trigger.data.0 else {
                        unreachable!()
                    };

                    let Some(InputValue::OffChain(OffChainInputValue::Number(id))) =
                        InputType::OffChain(OffChainInputType::Number(16))
                            .parse_value(data.get(link).unwrap())
                    else {
                        unreachable!()
                    };

                    links.push((link, id as u32));
                }
            }

            for action in script.0.off_chain_actions.iter() {
                if let Some(link) = action.action_kind.raw_input_schema().link {
                    let ActionInputData::Raw { data } = &action.data.0 else {
                        unreachable!()
                    };

                    let Some(InputValue::OffChain(OffChainInputValue::Number(id))) =
                        InputType::OffChain(OffChainInputType::Number(16))
                            .parse_value(data.get(link).unwrap())
                    else {
                        unreachable!()
                    };

                    links.push((link, id as u32));
                }
            }
        }

        links
    }

    pub fn export_scripts(self) -> Vec<Script> {
        self.scripts.into_iter().map(|s| s.0.get_model()).collect()
    }

    pub fn is_on_chain(&self) -> bool {
        for script in self.scripts.iter() {
            if !script.0.on_chain_actions.is_empty() {
                return true;
            }
        }

        false
    }
}

#[async_trait::async_trait]
impl ValidateParsed for ParsedScript {
    type Error = Error;
    type Metadata = (Option<H160>, ChainId);
    type Result = Valid<ParsedScript>;

    async fn validate(
        self,
        (input_token, chain_id): Self::Metadata,
    ) -> Result<Self::Result, Self::Error> {
        let mut input_token = input_token;
        for action in self.on_chain_actions.iter() {
            if action.action_kind.group() == ActionGroup::OffChain {
                return Err(Error::InvalidActionGroup);
            }

            if input_token.is_none() {
                return Err(Error::InvalidActionSequence);
            }

            input_token = (action.action_kind.output_calculation().unwrap())(
                &action.data,
                input_token.unwrap(),
                chain_id,
            )
            .await;
        }

        for action in self.off_chain_actions.iter() {
            if action.action_kind.group() == ActionGroup::OnChain {
                return Err(Error::InvalidActionGroup);
            }
        }

        if self.triggers.is_empty() {
            return Err(Error::EmptyTriggersList);
        }

        if self.on_chain_actions.is_empty() && self.off_chain_actions.is_empty() {
            return Err(Error::EmptyActionsList);
        }

        Ok(Valid(self))
    }
}

#[async_trait::async_trait]
impl ValidateParsed for ParsedScenario {
    type Error = Error;
    type Metadata = ();
    type Result = ValidParsedScenario;

    async fn validate(self, _: Self::Metadata) -> Result<Self::Result, Self::Error> {
        let mut scripts = Vec::with_capacity(self.scripts.len());
        for script in self.scripts.into_iter() {
            scripts.push(script.validate((self.input_token, self.chain_id)).await?);
        }

        let mut on_chain = true;
        for script in scripts.iter() {
            if script.0.on_chain_actions.is_empty() {
                on_chain = false;
            } else if !on_chain {
                return Err(Error::InvalidScriptSequence);
            }
        }

        Ok(ValidParsedScenario {
            id: self.id,
            chain_id: self.chain_id,
            owner: self.owner,
            actor: self.actor,
            input_token: self.input_token,
            input_amount: self.input_amount,
            scripts,
        })
    }
}
