use ethabi::Address;
use std::collections::HashMap;

use super::SourceKind;
use crate::ChainId;

pub type SourceKindType = dyn SourceKind + Send + Sync;

pub struct Source {
    pub name: String,
    addresses: HashMap<ChainId, Address>,
    kinds: Vec<Box<SourceKindType>>,
}

impl Source {
    pub(crate) fn builder(name: String) -> Self {
        Self {
            name,
            addresses: HashMap::new(),
            kinds: Vec::new(),
        }
    }

    pub(crate) fn add_address(mut self, chain_id: ChainId, address: Address) -> Self {
        self.addresses.insert(chain_id, address);

        self
    }

    pub(crate) fn add_kind(mut self, kind: Box<SourceKindType>) -> Self {
        self.kinds.push(kind);

        self
    }

    pub fn get_address(&self, chain_id: ChainId) -> Option<&Address> {
        self.addresses.get(&chain_id)
    }

    #[allow(clippy::borrowed_box)]
    pub fn kind_by_name(&self, name: &str) -> Option<&Box<SourceKindType>> {
        self.kinds.iter().find(|kind| kind.name() == name)
    }

    #[allow(clippy::borrowed_box)]
    pub fn kind_by_kind(&self, kind_code: u8) -> Option<&Box<SourceKindType>> {
        self.kinds.iter().find(|kind| {
            kind.on_chain_input_schema()
                .filter(|v| v.kind == kind_code)
                .is_some()
        })
    }
}

pub struct Sources {
    sources: HashMap<String, Source>,
}

impl Sources {
    pub(crate) fn builder() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub(crate) fn add_source(mut self, value: Source) -> Self {
        self.sources.insert(value.name.clone(), value);

        self
    }

    pub fn source_by_name(&self, key: &str) -> Option<&Source> {
        self.sources.get(key)
    }

    pub fn source_by_addr(&self, addr: Address, chain_id: ChainId) -> Option<&Source> {
        self.sources
            .iter()
            .find(|(_, source)| source.addresses.get(&chain_id) == Some(&addr))
            .map(|(_, source)| source)
    }
}
