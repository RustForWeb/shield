use async_trait::async_trait;

use shield::{Session, Shield, User};

#[async_trait]
pub trait LeptosIntegration<U: User>: Send + Sync {
    async fn extract_shield(&self) -> Shield<U>;

    async fn extract_session(&self) -> Session;

    fn redirect(&self, path: &str);
}
