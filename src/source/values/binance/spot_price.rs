use std::collections::{HashMap, HashSet};

use crate::{
    input::{InputType, OffChainInputType},
    source::{
        data::{OnChainInputSchema, RawInputSchema},
        SourceGroup, SourceKind, SourceKindInfo,
    },
};

pub struct SpotPriceSource {
    raw_schema: RawInputSchema,
}

pub fn boxed() -> Box<SpotPriceSource> {
    Box::new(SpotPriceSource {
        raw_schema: RawInputSchema {
            data: HashMap::from([
                (
                    "left_token".to_owned(),
                    InputType::OffChain(OffChainInputType::String),
                ),
                (
                    "right_token".to_owned(),
                    InputType::OffChain(OffChainInputType::String),
                ),
            ]),
            condition: HashSet::from([0, 1]),
            link: Some("binance"),
        },
    })
}

impl SourceKindInfo for SpotPriceSource {
    fn name(&self) -> &'static str {
        "datetime"
    }

    fn group(&self) -> SourceGroup {
        SourceGroup::OffChain
    }

    fn raw_input_schema(&self) -> &RawInputSchema {
        &self.raw_schema
    }

    fn on_chain_input_schema(&self) -> Option<&OnChainInputSchema> {
        None
    }

    fn filters(&self) -> Option<&[crate::source::SourceFilter]> {
        None
    }
}

impl SourceKind for SpotPriceSource {}
