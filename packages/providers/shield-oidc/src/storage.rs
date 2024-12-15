use async_trait::async_trait;

use shield::{Storage, StorageError};

use crate::subprovider::OidcSubprovider;

#[async_trait]
pub trait OidcStorage: Storage + Sync {
    async fn oidc_subproviders(&self) -> Result<Vec<OidcSubprovider>, StorageError>;

    async fn oidc_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<OidcSubprovider>, StorageError>;
}
