use ethabi::{Bytes, ParamType};
use std::collections::{HashMap, HashSet};

use crate::input::{InputType, OnChainInputType};

#[derive(Debug, Clone)]
pub struct RawInputSchema {
    pub data: HashMap<String, InputType>,
    pub condition: HashSet<u8>,
    pub link: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct OnChainFieldSchema {
    pub parameter: ParamType,
    pub input_type: OnChainInputType,
    pub field: String,
}

#[derive(Debug, Clone)]
pub struct OnChainInputSchema {
    pub kind: u8,
    pub data: Vec<OnChainFieldSchema>,
}

impl OnChainInputSchema {
    pub fn extract_params(&self) -> Vec<ParamType> {
        self.data
            .iter()
            .map(|v| v.parameter.clone())
            .collect::<Vec<ParamType>>()
    }
}

#[derive(Debug, Clone)]
pub enum InputData {
    Raw {
        data: HashMap<String, String>,
        condition: u8,
    },
    OnChain {
        input: Bytes,
        condition: u8,
    },
}
