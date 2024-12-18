use crate::{session::Session, shield::Shield};

pub trait ClientIntegration {}

pub trait ServerIntegration {
    fn extract_shield() -> Shield;

    fn extract_session() -> Session;
}
