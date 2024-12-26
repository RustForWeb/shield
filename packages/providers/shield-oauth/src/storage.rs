use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::subprovider::OauthSubprovider;

#[async_trait]
pub trait OauthStorage<U: User>: Storage<U> + Sync {
    async fn oauth_subproviders(&self) -> Result<Vec<OauthSubprovider>, StorageError>;

    async fn oauth_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<OauthSubprovider>, StorageError>;
}
