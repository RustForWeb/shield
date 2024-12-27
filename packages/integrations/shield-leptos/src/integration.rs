use async_trait::async_trait;

use shield::{Session, ShieldDyn};

#[async_trait]
pub trait LeptosIntegration: Send + Sync {
    async fn extract_shield(&self) -> ShieldDyn;

    async fn extract_session(&self) -> Session;

    fn redirect(&self, path: &str);
}
