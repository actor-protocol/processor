use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone)]
pub struct LinkInputDescription {
    pub inputs: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct LinkInputData {
    pub inputs: HashMap<String, String>,
}
