use async_trait::async_trait;

use crate::{session::Session, shield::Shield, user::User};

pub trait ClientIntegration {}

#[async_trait]
pub trait ServerIntegration<U: User>: Send + Sync {
    async fn extract_shield(&self) -> Shield<U>;

    async fn extract_session(&self) -> Session;

    fn redirect(&self, path: &str);
}
