use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::StorageError;
use shield_oidc::{
    CreateOidcConnection, OidcConnection, OidcStorage, OidcSubprovider, UpdateOidcConnection,
};
use uuid::Uuid;

use crate::{storage::MemoryStorage, user::User};

#[derive(Clone, Debug, Default)]
pub struct OidcMemoryStorage {
    connections: Arc<Mutex<Vec<OidcConnection>>>,
}

#[async_trait]
impl OidcStorage<User> for MemoryStorage {
    async fn oidc_subproviders(&self) -> Result<Vec<OidcSubprovider>, StorageError> {
        Ok(vec![])
    }

    async fn oidc_subprovider_by_id(
        &self,
        _subprovider_id: &str,
    ) -> Result<Option<OidcSubprovider>, StorageError> {
        Ok(None)
    }

    async fn oidc_connection_by_id(
        &self,
        connection_id: &str,
    ) -> Result<Option<OidcConnection>, StorageError> {
        Ok(self
            .oidc
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|connection| connection.id == connection_id)
            .cloned())
    }

    async fn oidc_connection_by_identifier(
        &self,
        subprovider_id: &str,
        identifier: &str,
    ) -> Result<Option<OidcConnection>, StorageError> {
        Ok(self
            .oidc
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|connection| {
                connection.subprovider_id == subprovider_id && connection.identifier == identifier
            })
            .cloned())
    }

    async fn create_oidc_connection(
        &self,
        connection: CreateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        let connection = OidcConnection {
            id: Uuid::new_v4().to_string(),
            identifier: connection.identifier,
            token_type: connection.token_type,
            access_token: connection.access_token,
            refresh_token: connection.refresh_token,
            id_token: connection.id_token,
            expired_at: connection.expired_at,
            scopes: connection.scopes,
            subprovider_id: connection.subprovider_id,
            user_id: connection.user_id,
        };

        self.oidc
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .push(connection.clone());

        Ok(connection)
    }

    async fn update_oidc_connection(
        &self,
        connection: UpdateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        let mut connections = self
            .oidc
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
        if let Some(id_token) = connection.id_token {
            connection_mut.id_token = id_token;
        }
        if let Some(expired_at) = connection.expired_at {
            connection_mut.expired_at = expired_at;
        }
        if let Some(scopes) = connection.scopes {
            connection_mut.scopes = scopes;
        }

        Ok(connection_mut.clone())
    }

    async fn delete_oidc_connection(&self, connection_id: &str) -> Result<(), StorageError> {
        self.oidc
            .connections
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .retain(|connection| connection.id != connection_id);

        Ok(())
    }
}
