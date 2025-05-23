use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::{
    connection::{CreateOauthConnection, OauthConnection, UpdateOauthConnection},
    provider::OauthProvider,
};

#[async_trait]
pub trait OauthStorage<U: User>: Storage<U> + Sync {
    async fn oauth_providers(&self) -> Result<Vec<OauthProvider>, StorageError>;

    async fn oauth_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<Option<OauthProvider>, StorageError>;

    async fn oauth_connection_by_id(
        &self,
        connection_id: &str,
    ) -> Result<Option<OauthConnection>, StorageError>;

    async fn oauth_connection_by_identifier(
        &self,
        provider_id: &str,
        identifier: &str,
    ) -> Result<Option<OauthConnection>, StorageError>;

    async fn create_oauth_connection(
        &self,
        connection: CreateOauthConnection,
    ) -> Result<OauthConnection, StorageError>;

    async fn update_oauth_connection(
        &self,
        connection: UpdateOauthConnection,
    ) -> Result<OauthConnection, StorageError>;

    async fn delete_oauth_connection(&self, connection_id: &str) -> Result<(), StorageError>;
}
