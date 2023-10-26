use super::storage::{Link, Links};
use std::sync::OnceLock;

pub mod email;
pub mod telegram;

pub fn get_links() -> &'static Links {
    static LINKS: OnceLock<Links> = OnceLock::new();
    LINKS.get_or_init(|| {
        Links::builder()
            .add_link(Link::builder(email::boxed()))
            .add_link(Link::builder(telegram::boxed()))
    })
}
