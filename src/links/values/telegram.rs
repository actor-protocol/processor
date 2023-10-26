use model::link::LinkStatus;
use rand::distributions::{Alphanumeric, DistString};

use crate::links::{
    data::{LinkInputData, LinkInputDescription},
    ConfirmLinking, LinkConfirmation, LinkInfo, LinkKind, ValidateLinkInputData, Validator,
};

#[derive(Default)]
pub struct TelegramLink(LinkInputDescription);

impl LinkInfo for TelegramLink {
    fn name(&self) -> &'static str {
        "telegram"
    }
}

impl ValidateLinkInputData for TelegramLink {
    fn input_description(&self) -> &LinkInputDescription {
        &self.0
    }

    fn validators(&self) -> Option<&Vec<Validator>> {
        None
    }
}

impl ConfirmLinking for TelegramLink {
    fn confirm_link(&self, _: LinkInputData) -> LinkConfirmation {
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        LinkConfirmation {
            status: LinkStatus::Waiting,
            confirmation: Some(token),
            value: None,
            label: None,
        }
    }
}

impl LinkKind for TelegramLink {}

pub fn boxed() -> Box<TelegramLink> {
    Box::<TelegramLink>::default()
}
