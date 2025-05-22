use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::{
    connection::{CreateOidcConnection, OidcConnection, UpdateOidcConnection},
    provider::OidcProvider,
};

#[async_trait]
pub trait OidcStorage<U: User>: Storage<U> + Sync {
    async fn oidc_providers(&self) -> Result<Vec<OidcProvider>, StorageError>;

    async fn oidc_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<Option<OidcProvider>, StorageError>;

    async fn oidc_connection_by_id(
        &self,
        connection_id: &str,
    ) -> Result<Option<OidcConnection>, StorageError>;

    async fn oidc_connection_by_identifier(
        &self,
        provider_id: &str,
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

    async fn delete_oidc_connection(&self, connection_id: &str) -> Result<(), StorageError>;
}
