use std::sync::Arc;

use async_trait::async_trait;
use shield::{Session, ShieldDyn, User};

#[async_trait]
pub trait LeptosIntegration: Send + Sync {
    async fn extract_shield(&self) -> ShieldDyn;

    async fn extract_session(&self) -> Session;

    async fn extract_user(&self) -> Option<Arc<dyn User>>;

    fn redirect(&self, path: &str);
}
