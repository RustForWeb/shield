use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::StorageError;
use shield_oidc::{
    CreateOidcConnection, OidcConnection, OidcStorage, OidcSubprovider, UpdateOidcConnection,
};

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
        _connection: CreateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        todo!("create_oidc_connection")
    }

    async fn update_oidc_connection(
        &self,
        _connection: UpdateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        todo!("update_oidc_connection")
    }

    async fn delete_oidc_connection(&self, _connection_id: &str) -> Result<(), StorageError> {
        todo!("delete_oidc_connection")
    }
}
