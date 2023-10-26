use crate::{
    input::{InputType, InputValue, OffChainInputType},
    ChainId, Filtered, Valid,
};

use data::{InputData, OnChainInputSchema, RawInputSchema};
use ethabi::{ethereum_types::H160, Token};
use std::{collections::HashMap, fmt::Display, future::Future, pin::Pin};

pub mod data;
pub mod storage;
pub mod values;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionGroup {
    OnChain,
    OffChain,
}

impl Display for ActionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActionGroup::OnChain => "on-chain",
                ActionGroup::OffChain => "off-chain",
            }
        )
    }
}

pub type OutputFn = Box<
    dyn Fn(
            &Valid<InputData>, // Raw data
            H160,              // Input token
            ChainId,           // Chain id
        ) -> Pin<Box<dyn Future<Output = Option<H160>> + Send + Sync>>
        + Send
        + Sync,
>;

pub type ActionFilter =
    Box<dyn for<'a> Fn(&'a Valid<InputData>, Option<ChainId>) -> bool + Send + Sync>;

pub trait ActionKindInfo {
    fn name(&self) -> &'static str;
    fn group(&self) -> ActionGroup;

    fn raw_input_schema(&self) -> &RawInputSchema;
    fn on_chain_input_schema(&self) -> Option<&OnChainInputSchema>;
    fn output_calculation(&self) -> Option<OutputFn>;

    fn filters(&self) -> Option<&[ActionFilter]>;
}

pub trait ValidateActionInput: ActionKindInfo {
    fn validate_input_data(&self, input_data: InputData) -> Option<Valid<InputData>> {
        match input_data {
            InputData::Raw { ref data } => {
                let schema = self.raw_input_schema();

                // Validate HashMap length.
                if schema.data.len() + {
                    if schema.link.is_some() {
                        1
                    } else {
                        0
                    }
                } != data.len()
                {
                    return None;
                }

                // Validate link parameter.
                if let Some(link) = schema.link {
                    let field = data.get(link)?;
                    InputType::OffChain(OffChainInputType::Number(16)).parse_value(field)?;
                }

                // Validate each data field.
                for (key, input_type) in schema.data.iter() {
                    let field = data.get(key)?;
                    input_type.parse_value(field)?;
                }

                Some(Valid(input_data))
            }
            InputData::OnChain { ref input } => {
                let schema = self.on_chain_input_schema()?;

                // Convert input to the tokens.
                let tokens =
                    ethabi::decode(schema.extract_params().as_slice(), input.as_slice()).ok()?;

                // Validate each token.
                for (idx, token) in tokens.into_iter().enumerate() {
                    let field = schema.data.get(idx)?;
                    if !token.type_check(&field.parameter)
                        || !field.input_type.validate_token(&token)
                    {
                        return None;
                    }
                }

                Some(Valid(input_data))
            }
        }
    }
}

impl<T: ActionKindInfo> ValidateActionInput for T {}

pub trait TransformActionInput: ActionKindInfo {
    fn transform_input_data(
        &self,
        schema: &OnChainInputSchema,
        input_data: Valid<InputData>,
    ) -> Valid<InputData> {
        match input_data.0 {
            InputData::Raw { data } => {
                let tokens: Vec<Token> = schema
                    .data
                    .iter()
                    .map(|field| {
                        let InputValue::OnChain(v) = InputType::OnChain(field.input_type)
                            .parse_value(data.get(&field.field).unwrap())
                            .unwrap()
                        else {
                            unreachable!();
                        };

                        v.tokenize()
                    })
                    .collect();

                Valid(InputData::OnChain {
                    input: ethabi::encode(tokens.as_slice()),
                })
            }
            InputData::OnChain { input } => {
                let tokens =
                    ethabi::decode(schema.extract_params().as_slice(), input.as_slice()).unwrap();

                let mut data = HashMap::new();
                for (idx, input) in schema.data.iter().enumerate() {
                    data.insert(
                        input.field.clone(),
                        InputValue::OnChain(
                            input.input_type.parse_token(tokens[idx].clone()).unwrap(),
                        )
                        .stringify(),
                    );
                }

                Valid(InputData::Raw { data })
            }
        }
    }
}

impl<T: ActionKindInfo> TransformActionInput for T {}

pub trait FilterActionInput: ActionKindInfo + TransformActionInput {
    fn filter_input_data(
        &self,
        input_data: Valid<InputData>,
        chain_id: Option<ChainId>,
    ) -> Option<Filtered<Valid<InputData>>> {
        let Some(filters) = self.filters() else {
            return Some(Filtered(input_data));
        };

        let data = match &input_data.0 {
            InputData::Raw { ref data } => Valid(InputData::Raw { data: data.clone() }),
            InputData::OnChain { ref input } => self.transform_input_data(
                self.on_chain_input_schema()?,
                Valid(InputData::OnChain {
                    input: input.clone(),
                }),
            ),
        };

        for filter in filters.iter() {
            if !(filter)(&data, chain_id) {
                return None;
            }
        }

        Some(Filtered(input_data))
    }
}

impl<T: ActionKindInfo> FilterActionInput for T {}

pub trait ActionKind:
    ActionKindInfo + ValidateActionInput + TransformActionInput + FilterActionInput
{
}
