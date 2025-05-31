use async_trait::async_trait;
use serde::de::DeserializeOwned;
use shield::{Form, ShieldError, User};

#[async_trait]
pub trait Credentials<U: User, D: DeserializeOwned>: Send + Sync {
    fn form(&self) -> Form;

    async fn sign_in(&self, data: D) -> Result<U, ShieldError>;
}
