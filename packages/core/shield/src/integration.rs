use async_trait::async_trait;

use crate::{session::Session, shield::Shield};

pub trait ClientIntegration {}

#[async_trait]
pub trait ServerIntegration: Send + Sync {
    async fn extract_shield(&self) -> Shield;

    async fn extract_session(&self) -> Session;

    fn redirect(&self, path: &str);
}
