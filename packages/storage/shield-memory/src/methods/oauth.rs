use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::StorageError;
use shield_oauth::{
    CreateOauthConnection, OauthConnection, OauthProvider, OauthStorage, UpdateOauthConnection,
};
use uuid::Uuid;

use crate::{storage::MemoryStorage, user::User};

#[derive(Clone, Debug, Default)]
pub struct OauthMemoryStorage {
    connections: Arc<Mutex<Vec<OauthConnection>>>,
}

#[async_trait]
impl OauthStorage<User> for MemoryStorage {
    async fn oauth_providers(&self) -> Result<Vec<OauthProvider>, StorageError> {
        Ok(vec![])
    }

    async fn oauth_provider_by_id_or_slug(
        &self,
        _provider_id: &str,
    ) -> Result<Option<OauthProvider>, StorageError> {
        Ok(None)
    }

    async fn oauth_connection_by_id(
        &self,
        connection_id: &str,
    ) -> Result<Option<OauthConnection>, StorageError> {
        Ok(self
            .oauth
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|connection| connection.id == connection_id)
            .cloned())
    }

    async fn oauth_connection_by_identifier(
        &self,
        provider_id: &str,
        identifier: &str,
    ) -> Result<Option<OauthConnection>, StorageError> {
        Ok(self
            .oauth
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|connection| {
                connection.provider_id == provider_id && connection.identifier == identifier
            })
            .cloned())
    }

    async fn create_oauth_connection(
        &self,
        connection: CreateOauthConnection,
    ) -> Result<OauthConnection, StorageError> {
        let connection = OauthConnection {
            id: Uuid::new_v4().to_string(),
            identifier: connection.identifier,
            token_type: connection.token_type,
            access_token: connection.access_token,
            refresh_token: connection.refresh_token,
            expired_at: connection.expired_at,
            scopes: connection.scopes,
            provider_id: connection.provider_id,
            user_id: connection.user_id,
        };

        self.oauth
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .push(connection.clone());

        Ok(connection)
    }

    async fn update_oauth_connection(
        &self,
        connection: UpdateOauthConnection,
    ) -> Result<OauthConnection, StorageError> {
        let mut connections = self
            .oauth
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?;

        let connection_mut = connections
            .iter_mut()
            .find(|c| c.id == connection.id)
            .ok_or_else(|| StorageError::NotFound("User".to_owned(), connection.id.clone()))?;

        if let Some(token_type) = connection.token_type {
            connection_mut.token_type = token_type;
        }
        if let Some(access_token) = connection.access_token {
            connection_mut.access_token = access_token;
        }
        if let Some(refresh_token) = connection.refresh_token {
            connection_mut.refresh_token = refresh_token;
        }
        if let Some(expired_at) = connection.expired_at {
            connection_mut.expired_at = expired_at;
        }
        if let Some(scopes) = connection.scopes {
            connection_mut.scopes = scopes;
        }

        Ok(connection_mut.clone())
    }

    async fn delete_oauth_connection(&self, connection_id: &str) -> Result<(), StorageError> {
        self.oauth
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .retain(|connection| connection.id != connection_id);

        Ok(())
    }
}
