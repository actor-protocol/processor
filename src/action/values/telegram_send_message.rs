use std::collections::HashMap;

use crate::{
    action::{
        data::{OnChainInputSchema, RawInputSchema},
        ActionGroup, ActionKind, ActionKindInfo,
    },
    input::{InputType, OffChainInputType},
};

pub struct TelegramSendMessageAction {
    raw_schema: RawInputSchema,
}

pub fn boxed() -> Box<TelegramSendMessageAction> {
    Box::new(TelegramSendMessageAction {
        raw_schema: RawInputSchema {
            data: HashMap::from([(
                "message".to_owned(),
                InputType::OffChain(OffChainInputType::String),
            )]),
            link: Some("telegram"),
        },
    })
}

impl ActionKindInfo for TelegramSendMessageAction {
    fn name(&self) -> &'static str {
        "telegram-send-message"
    }

    fn group(&self) -> ActionGroup {
        ActionGroup::OffChain
    }

    fn raw_input_schema(&self) -> &RawInputSchema {
        &self.raw_schema
    }

    fn on_chain_input_schema(&self) -> Option<&OnChainInputSchema> {
        None
    }

    fn output_calculation(&self) -> Option<crate::action::OutputFn> {
        None
    }

    fn filters(&self) -> Option<&[crate::action::ActionFilter]> {
        None
    }
}

impl ActionKind for TelegramSendMessageAction {}
