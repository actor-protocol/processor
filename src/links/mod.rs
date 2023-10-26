pub mod data;
pub mod storage;
pub mod values;

use data::{LinkInputData, LinkInputDescription};
use model::link::LinkStatus;

pub type Validator = Box<dyn Fn(LinkInputData) -> bool + Send + Sync>;

pub trait LinkInfo {
    fn name(&self) -> &'static str;
}

pub trait ValidateLinkInputData {
    fn input_description(&self) -> &LinkInputDescription;
    fn validators(&self) -> Option<&Vec<Validator>>;

    fn validate_input(&self, data: LinkInputData) -> bool {
        let description = self.input_description();

        if data.inputs.len() != description.inputs.len() {
            return false;
        }

        for (key, _) in data.inputs.iter() {
            if !description.inputs.contains(key) {
                return false;
            }
        }

        if let Some(validators) = self.validators() {
            for validator in validators.iter() {
                if !(validator)(data.clone()) {
                    return false;
                }
            }
        }

        true
    }
}

pub struct LinkConfirmation {
    pub status: LinkStatus,
    pub confirmation: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

pub trait ConfirmLinking {
    fn confirm_link(&self, data: LinkInputData) -> LinkConfirmation;
}

pub trait LinkKind: LinkInfo + ValidateLinkInputData + ConfirmLinking {}
