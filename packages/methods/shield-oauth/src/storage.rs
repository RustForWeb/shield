use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::provider::OauthProvider;

#[async_trait]
pub trait OauthStorage<U: User>: Storage<U> + Sync {
    async fn oauth_providers(&self) -> Result<Vec<OauthProvider>, StorageError>;

    async fn oauth_provider_by_id(
        &self,
        provider_id: &str,
    ) -> Result<Option<OauthProvider>, StorageError>;
}
