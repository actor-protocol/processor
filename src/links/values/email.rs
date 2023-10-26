use model::link::LinkStatus;
use std::collections::HashSet;

use crate::links::{
    data::{LinkInputData, LinkInputDescription},
    ConfirmLinking, LinkConfirmation, LinkInfo, LinkKind, ValidateLinkInputData, Validator,
};

pub struct EmailLink {
    input_description: LinkInputDescription,
    validators: Vec<Validator>,
}

impl EmailLink {
    fn new() -> Self {
        Self {
            input_description: LinkInputDescription {
                inputs: HashSet::from(["email".to_owned()]),
            },
            validators: vec![Box::new(|input| {
                validator::validate_email(input.inputs.get("email").unwrap())
            })],
        }
    }
}

impl LinkInfo for EmailLink {
    fn name(&self) -> &'static str {
        "email"
    }
}

impl ValidateLinkInputData for EmailLink {
    fn input_description(&self) -> &LinkInputDescription {
        &self.input_description
    }

    fn validators(&self) -> Option<&Vec<Validator>> {
        Some(&self.validators)
    }
}

impl ConfirmLinking for EmailLink {
    fn confirm_link(&self, data: LinkInputData) -> LinkConfirmation {
        let email = data.inputs.get("email").unwrap();

        LinkConfirmation {
            status: LinkStatus::Connected,
            confirmation: None,
            value: Some(email.clone()),
            label: Some(email.clone()),
        }
    }
}

impl LinkKind for EmailLink {}

pub fn boxed() -> Box<EmailLink> {
    Box::new(EmailLink::new())
}
