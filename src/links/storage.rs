use super::LinkKind;
use std::collections::HashMap;

pub type LinkType = dyn LinkKind + Send + Sync;

pub struct Link {
    pub name: String,
    kind: Box<LinkType>,
}

impl Link {
    pub(crate) fn builder(kind: Box<LinkType>) -> Self {
        Self {
            name: kind.name().to_string(),
            kind,
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self) -> &Box<LinkType> {
        &self.kind
    }
}

pub struct Links {
    links: HashMap<String, Link>,
}

impl Links {
    pub(crate) fn builder() -> Self {
        Self {
            links: HashMap::default(),
        }
    }

    pub(crate) fn add_link(mut self, value: Link) -> Self {
        self.links.insert(value.name.clone(), value);

        self
    }

    pub fn get_link(&self, key: &str) -> Option<&Link> {
        self.links.get(key)
    }
}
