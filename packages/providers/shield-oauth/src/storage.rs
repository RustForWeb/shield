use async_trait::async_trait;

use shield::{Storage, StorageError};

use crate::provider::OauthSubprovider;

#[async_trait]
pub trait OauthStorage: Storage + Sync {
    async fn oauth_subproviders(&self) -> Result<Vec<OauthSubprovider>, StorageError>;

    async fn oauth_subprovider_by_id(
        &self,
        subprovider_id: String,
    ) -> Result<Option<OauthSubprovider>, StorageError>;
}
