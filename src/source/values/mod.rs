use super::storage::Sources;
use crate::source::storage::Source;

use ethabi::ethereum_types::H160;
use std::{str::FromStr, sync::OnceLock};

mod binance;
mod time_based;

pub fn get_sources() -> &'static Sources {
    static SOURCES: OnceLock<Sources> = OnceLock::new();

    SOURCES.get_or_init(|| {
        Sources::builder()
            .add_source(
                Source::builder("timebased".to_owned())
                    .add_address(
                        80001,
                        H160::from_str("0xB87EcDa1b08D0D2677B822C062FF33F605F5f73e").unwrap(),
                    )
                    .add_kind(time_based::date_time::boxed())
                    .add_kind(time_based::time_of_day::boxed()),
            )
            .add_source(
                Source::builder("binance".to_owned()).add_kind(binance::spot_price::boxed()),
            )
    })
}
