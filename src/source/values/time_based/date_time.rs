use ethabi::ParamType;
use std::collections::{HashMap, HashSet};

use crate::{
    input::{InputType, OffChainInputType, OnChainInputType},
    source::{
        data::{OnChainFieldSchema, OnChainInputSchema, RawInputSchema},
        SourceGroup, SourceKind, SourceKindInfo,
    },
};

pub struct DateTimeSource {
    raw_schema: RawInputSchema,
    on_chain_input_schema: OnChainInputSchema,
}

pub fn boxed() -> Box<DateTimeSource> {
    Box::new(DateTimeSource {
        raw_schema: RawInputSchema {
            data: HashMap::from([(
                "timestamp".to_owned(),
                InputType::OffChain(OffChainInputType::Number(32)),
            )]),
            condition: HashSet::from([0, 1]),
            link: None,
        },
        on_chain_input_schema: OnChainInputSchema {
            kind: 1,
            data: vec![OnChainFieldSchema {
                parameter: ParamType::Uint(256),
                input_type: OnChainInputType::U256(32),
                field: "timestamp".to_owned(),
            }],
        },
    })
}

impl SourceKindInfo for DateTimeSource {
    fn name(&self) -> &'static str {
        "datetime"
    }

    fn group(&self) -> SourceGroup {
        SourceGroup::Both
    }

    fn raw_input_schema(&self) -> &RawInputSchema {
        &self.raw_schema
    }

    fn on_chain_input_schema(&self) -> Option<&OnChainInputSchema> {
        Some(&self.on_chain_input_schema)
    }

    fn filters(&self) -> Option<&[crate::source::SourceFilter]> {
        None
    }
}

impl SourceKind for DateTimeSource {}
