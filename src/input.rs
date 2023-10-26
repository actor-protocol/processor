use ethabi::{
    ethereum_types::{H160, U256},
    Bytes, Token,
};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum OnChainInputType {
    Bytes,
    Address,
    U256(u16),
}

impl OnChainInputType {
    pub fn validate_token(&self, value: &Token) -> bool {
        match self {
            OnChainInputType::Bytes => {
                if let Token::Bytes(_) = value {
                    return true;
                }
            }
            OnChainInputType::Address => {
                if let Token::Address(_) = value {
                    return true;
                }
            }
            OnChainInputType::U256(bits) => {
                if let Token::Uint(v) = value {
                    if v.bits() <= *bits as usize {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn parse_token(&self, value: Token) -> Option<OnChainInputValue> {
        match self {
            OnChainInputType::Bytes => {
                if let Token::Bytes(v) = value {
                    return Some(OnChainInputValue::Bytes(v));
                }
            }
            OnChainInputType::Address => {
                if let Token::Address(v) = value {
                    return Some(OnChainInputValue::Address(v));
                }
            }
            OnChainInputType::U256(bits) => {
                if let Token::Uint(v) = value {
                    if v.bits() <= *bits as usize {
                        return Some(OnChainInputValue::U256(v));
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OffChainInputType {
    String,
    Number(u8),
    Float,
}

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    OnChain(OnChainInputType),
    OffChain(OffChainInputType),
}

impl InputType {
    pub fn parse_value(&self, value: &str) -> Option<InputValue> {
        match self {
            InputType::OnChain(t) => match t {
                OnChainInputType::Bytes => hex::decode(value)
                    .ok()
                    .map(OnChainInputValue::Bytes)
                    .map(InputValue::OnChain),
                OnChainInputType::Address => H160::from_str(value)
                    .ok()
                    .map(OnChainInputValue::Address)
                    .map(InputValue::OnChain),
                OnChainInputType::U256(bits) => U256::from_dec_str(value)
                    .ok()
                    .and_then(|v| {
                        if v.bits() > *bits as usize {
                            None
                        } else {
                            Some(v)
                        }
                    })
                    .map(OnChainInputValue::U256)
                    .map(InputValue::OnChain),
            },
            InputType::OffChain(t) => match t {
                OffChainInputType::String => Some(InputValue::OffChain(
                    OffChainInputValue::String(value.to_owned()),
                )),
                OffChainInputType::Number(bits) => value
                    .parse::<u64>()
                    .ok()
                    .and_then(|v| {
                        if (64 - v.leading_zeros()) > *bits as u32 {
                            None
                        } else {
                            Some(v)
                        }
                    })
                    .map(OffChainInputValue::Number)
                    .map(InputValue::OffChain),
                OffChainInputType::Float => value
                    .parse::<f64>()
                    .ok()
                    .map(OffChainInputValue::Float)
                    .map(InputValue::OffChain),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum OnChainInputValue {
    Bytes(Bytes),
    Address(H160),
    U256(U256),
}

impl OnChainInputValue {
    pub fn tokenize(self) -> Token {
        match self {
            OnChainInputValue::Bytes(value) => Token::Bytes(value),
            OnChainInputValue::Address(value) => Token::Address(value),
            OnChainInputValue::U256(value) => Token::Uint(value),
        }
    }

    pub fn bytes(self) -> Option<Bytes> {
        match self {
            OnChainInputValue::Bytes(value) => Some(value),
            _ => None,
        }
    }

    pub fn address(self) -> Option<H160> {
        match self {
            OnChainInputValue::Address(value) => Some(value),
            _ => None,
        }
    }

    pub fn u256(self) -> Option<U256> {
        match self {
            OnChainInputValue::U256(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OffChainInputValue {
    String(String),
    Number(u64),
    Float(f64),
}

impl OffChainInputValue {
    pub fn string(self) -> Option<String> {
        match self {
            OffChainInputValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn number(self) -> Option<u64> {
        match self {
            OffChainInputValue::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn float(self) -> Option<f64> {
        match self {
            OffChainInputValue::Float(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputValue {
    OnChain(OnChainInputValue),
    OffChain(OffChainInputValue),
}

impl InputValue {
    pub fn stringify(&self) -> String {
        match self {
            InputValue::OnChain(v) => match v {
                OnChainInputValue::Bytes(value) => hex::encode(value),
                OnChainInputValue::Address(value) => {
                    let mut address = hex::encode(value.0);
                    address.insert_str(0, "0x");

                    address
                }
                OnChainInputValue::U256(value) => value.to_string(),
            },
            InputValue::OffChain(v) => match v {
                OffChainInputValue::String(v) => v.to_owned(),
                OffChainInputValue::Number(v) => v.to_string(),
                OffChainInputValue::Float(v) => v.to_string(),
            },
        }
    }

    pub fn on_chain(self) -> Option<OnChainInputValue> {
        match self {
            InputValue::OnChain(v) => Some(v),
            _ => None,
        }
    }

    pub fn off_chain(self) -> Option<OffChainInputValue> {
        match self {
            InputValue::OffChain(v) => Some(v),
            _ => None,
        }
    }
}
