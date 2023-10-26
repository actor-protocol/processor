use ethabi::ParamType;
use std::collections::HashMap;

use crate::{
    action::{
        data::{OnChainFieldSchema, OnChainInputSchema, RawInputSchema},
        ActionGroup, ActionKind, ActionKindInfo,
    },
    input::{InputType, OnChainInputType},
};

pub struct TransferAction {
    raw_schema: RawInputSchema,
    on_chain_schema: OnChainInputSchema,
}

pub fn boxed() -> Box<TransferAction> {
    Box::new(TransferAction {
        raw_schema: RawInputSchema {
            data: HashMap::from([(
                "target".to_owned(),
                InputType::OnChain(OnChainInputType::Address),
            )]),
            link: None,
        },
        on_chain_schema: OnChainInputSchema {
            data: vec![OnChainFieldSchema {
                parameter: ParamType::Address,
                input_type: OnChainInputType::Address,
                field: "target".to_owned(),
            }],
        },
    })
}

impl ActionKindInfo for TransferAction {
    fn name(&self) -> &'static str {
        "transfer"
    }

    fn group(&self) -> ActionGroup {
        ActionGroup::OnChain
    }

    fn raw_input_schema(&self) -> &RawInputSchema {
        &self.raw_schema
    }

    fn on_chain_input_schema(&self) -> Option<&OnChainInputSchema> {
        Some(&self.on_chain_schema)
    }

    fn output_calculation(&self) -> Option<crate::action::OutputFn> {
        Some(Box::new(|_, _, _| Box::pin(async { None })))
    }

    fn filters(&self) -> Option<&[crate::action::ActionFilter]> {
        None
    }
}

impl ActionKind for TransferAction {}
