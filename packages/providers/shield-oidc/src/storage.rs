use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::{
    connection::{CreateOidcConnection, OidcConnection, UpdateOidcConnection},
    subprovider::OidcSubprovider,
};

#[async_trait]
pub trait OidcStorage<U: User>: Storage<U> + Sync {
    async fn oidc_subproviders(&self) -> Result<Vec<OidcSubprovider>, StorageError>;

    async fn oidc_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<OidcSubprovider>, StorageError>;

    async fn oidc_connection_by_identifier(
        &self,
        subprovider_id: &str,
        identifier: &str,
    ) -> Result<Option<OidcConnection>, StorageError>;

    async fn create_oidc_connection(
        &self,
        connection: CreateOidcConnection,
    ) -> Result<OidcConnection, StorageError>;

    async fn update_oidc_connection(
        &self,
        connection: UpdateOidcConnection,
    ) -> Result<OidcConnection, StorageError>;
}
