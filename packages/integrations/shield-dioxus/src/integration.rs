use std::sync::Arc;

use async_trait::async_trait;
use shield::{Session, ShieldDyn};

#[async_trait]
pub trait DioxusIntegration: Send + Sync {
    async fn extract_shield(&self) -> ShieldDyn;

    async fn extract_session(&self) -> Session;
}

#[derive(Clone)]
pub struct DioxusIntegrationDyn(Arc<dyn DioxusIntegration>);

impl DioxusIntegrationDyn {
    pub fn new<I: DioxusIntegration + 'static>(integration: I) -> Self {
        Self(Arc::new(integration))
    }

    pub async fn extract_shield(&self) -> ShieldDyn {
        self.0.extract_shield().await
    }

    pub async fn extract_session(&self) -> Session {
        self.0.extract_session().await
    }
}
