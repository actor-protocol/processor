use ethabi::Bytes;
use ethers_core::abi::AbiEncode;

use crate::{
    action::{data::InputData as ActionInputData, ActionGroup},
    bindings::core_v1::{
        ActionData, AddScenarioCall, CoreV1Calls::AddScenario, Scenario, Script, SourceData,
    },
    source::{data::InputData as SourceInputData, SourceGroup},
    ChainId, Valid,
};

use super::{
    validator::ValidParsedScenario, Error, ParsedAction, ParsedScript, ParsedTrigger, TaggedError,
};

pub trait IntoCalldata {
    type Error;
    type Metadata;
    type Calldata;

    fn calldata(self, meta: Self::Metadata) -> Result<Self::Calldata, Self::Error>;
}

impl IntoCalldata for ParsedAction {
    type Error = Error;
    type Metadata = ChainId;
    type Calldata = ActionData;

    fn calldata(self, chain_id: Self::Metadata) -> Result<Self::Calldata, Self::Error> {
        let schema = self
            .action_kind
            .on_chain_input_schema()
            .ok_or(Error::MissingOnChainSchema)?;

        let Valid(ActionInputData::OnChain { input }) =
            self.action_kind.transform_input_data(schema, self.data)
        else {
            unreachable!()
        };

        Ok(ActionData {
            executor: *self
                .action
                .get_address(chain_id)
                .ok_or(Error::UnknownChainId)?,
            input: input.into(),
        })
    }
}

impl IntoCalldata for ParsedTrigger {
    type Error = Error;
    type Metadata = ChainId;
    type Calldata = SourceData;

    fn calldata(self, chain_id: Self::Metadata) -> Result<Self::Calldata, Self::Error> {
        let schema = self
            .source_kind
            .on_chain_input_schema()
            .ok_or(Error::MissingOnChainSchema)?;

        let Valid(SourceInputData::OnChain { input, condition }) =
            self.source_kind.transform_input_data(schema, self.data)
        else {
            unreachable!()
        };

        Ok(SourceData {
            addr: *self
                .source
                .get_address(chain_id)
                .ok_or(Error::UnknownChainId)?,
            kind: schema.kind,
            condition,
            input: input.into(),
        })
    }
}

impl IntoCalldata for Valid<ParsedScript> {
    type Error = TaggedError;
    type Metadata = ChainId;
    type Calldata = Script;

    fn calldata(self, chain_id: Self::Metadata) -> Result<Self::Calldata, Self::Error> {
        let script = self.0;

        if script.on_chain_actions.is_empty() {
            return Err(TaggedError::Error(Error::NoOnChainActions));
        }

        Ok(Script {
            trigger_type: script.trigger_type.as_u8(),
            sources_to_verify: script
                .triggers
                .into_iter()
                .filter(|v| {
                    v.on_chain_verification
                        && [SourceGroup::OnChain, SourceGroup::Both]
                            .contains(&v.source_kind.group())
                })
                .enumerate()
                .map(|(idx, v)| {
                    v.calldata(chain_id).map_err(|e| TaggedError::Trigger {
                        source_idx: idx as u32 + 1,
                        error: e,
                    })
                })
                .collect::<Result<Vec<SourceData>, TaggedError>>()?,
            actions_chain: script
                .on_chain_actions
                .into_iter()
                .enumerate()
                .map(|(idx, v)| {
                    v.calldata(chain_id).map_err(|e| TaggedError::Action {
                        action_idx: idx as u32 + 1,
                        group: ActionGroup::OnChain,
                        error: e,
                    })
                })
                .collect::<Result<Vec<ActionData>, TaggedError>>()?,
        })
    }
}

impl IntoCalldata for ValidParsedScenario {
    type Error = TaggedError;
    type Metadata = ();
    type Calldata = Bytes;

    fn calldata(self, _: Self::Metadata) -> Result<Self::Calldata, Self::Error> {
        let scenario = Scenario {
            owner: self.owner,
            actor: self
                .actor
                .ok_or(TaggedError::Error(Error::MissingActorAddress))?,
            input_token: self
                .input_token
                .ok_or(TaggedError::Error(Error::MissingInputToken))?,
            input_amount: self
                .input_amount
                .ok_or(TaggedError::Error(Error::MissingInputTokenAmount))?,
            scripts: self
                .scripts
                .into_iter()
                .enumerate()
                .map(|(idx, script)| {
                    script
                        .calldata(self.chain_id)
                        .map_err(|e| TaggedError::Scenario {
                            script_idx: idx as u32 + 1,
                            error: Box::new(e),
                        })
                })
                .collect::<Result<Vec<Script>, TaggedError>>()?,
        };

        Ok(AddScenario(AddScenarioCall {
            scenario,
            index_id: self.id as u64,
        })
        .encode())
    }
}
