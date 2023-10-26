use ethabi::Address;
use std::collections::HashMap;

use super::{ActionGroup, ActionKind};
use crate::ChainId;

pub type ActionKindType = dyn ActionKind + Send + Sync;

pub struct Action {
    pub name: String,
    addresses: HashMap<ChainId, Address>,
    kind: Box<ActionKindType>,
}

impl Action {
    pub(crate) fn builder(kind: Box<ActionKindType>) -> Self {
        match kind.group() {
            ActionGroup::OffChain => {
                assert!(kind.on_chain_input_schema().is_none());
                assert!(kind.output_calculation().is_none());
            }
            ActionGroup::OnChain => {
                assert!(kind.on_chain_input_schema().is_some());
                assert!(kind.output_calculation().is_some());
            }
        };

        Self {
            name: kind.name().to_owned(),
            addresses: HashMap::new(),
            kind,
        }
    }

    pub(crate) fn add_address(mut self, chain_id: ChainId, address: Address) -> Self {
        self.addresses.insert(chain_id, address);

        self
    }

    pub fn get_address(&self, chain_id: ChainId) -> Option<&Address> {
        self.addresses.get(&chain_id)
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self) -> &Box<ActionKindType> {
        &self.kind
    }
}

pub struct Actions {
    actions: HashMap<String, Action>,
}

impl Actions {
    pub(crate) fn builder() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub(crate) fn add_action(mut self, value: Action) -> Self {
        self.actions.insert(value.name.clone(), value);

        self
    }

    pub fn action_by_name(&self, key: &str) -> Option<&Action> {
        self.actions.get(key)
    }

    pub fn action_by_addr(&self, addr: Address, chain_id: ChainId) -> Option<&Action> {
        self.actions
            .iter()
            .find(|(_, action)| action.addresses.get(&chain_id) == Some(&addr))
            .map(|(_, action)| action)
    }
}
