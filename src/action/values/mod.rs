use ethabi::ethereum_types::H160;

use super::storage::Actions;
use crate::action::storage::Action;
use std::{str::FromStr, sync::OnceLock};

mod basic;
mod telegram_send_message;

pub fn get_actions() -> &'static Actions {
    static ACTIONS: OnceLock<Actions> = OnceLock::new();

    ACTIONS.get_or_init(|| {
        Actions::builder()
            .add_action(Action::builder(basic::transfer::boxed()).add_address(
                80001,
                H160::from_str("0x39aC82BaAbc1a86C27F9B04aaF085b143AB8F092").unwrap(),
            ))
            .add_action(Action::builder(telegram_send_message::boxed()))
    })
}
