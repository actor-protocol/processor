use std::fmt::Debug;

use ethabi::ethereum_types::H160;

pub mod action;
pub mod bindings;
pub mod input;
pub mod links;
pub mod parser;
pub mod source;

pub type ChainId = u32;

// Wrappers to make sure the code is called in the correct order
#[derive(Debug, Clone)]
pub struct Valid<T: Debug>(pub T);

#[derive(Debug, Clone)]
pub struct Filtered<T: Debug>(pub T);

pub fn format_address(value: H160) -> String {
    let mut address = hex::encode(value.0);
    address.insert_str(0, "0x");

    address
}
