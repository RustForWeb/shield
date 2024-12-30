use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shield::{Session, ShieldDyn, User};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeptosUser {
    pub id: String,
    pub name: Option<String>,
}

impl<U: User> From<U> for LeptosUser {
    fn from(value: U) -> Self {
        Self {
            id: value.id(),
            name: value.name(),
        }
    }
}

#[async_trait]
pub trait LeptosIntegration: Send + Sync {
    async fn extract_shield(&self) -> ShieldDyn;

    async fn extract_session(&self) -> Session;

    async fn extract_user(&self) -> Option<LeptosUser>;

    fn redirect(&self, path: &str);
}
