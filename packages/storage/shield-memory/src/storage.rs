use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::{CreateEmailAddress, CreateUser, Storage, StorageError, UpdateUser, User as _};

use crate::user::User;

pub const MEMORY_STORAGE_ID: &str = "memory";

#[derive(Clone, Debug, Default)]
pub struct MemoryStorage {
    pub(crate) users: Arc<Mutex<Vec<User>>>,
    #[cfg(feature = "provider-oidc")]
    pub(crate) oidc: crate::providers::oidc::OidcMemoryStorage,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage<User> for MemoryStorage {
    fn id(&self) -> String {
        MEMORY_STORAGE_ID.to_owned()
    }

    async fn user_by_id(&self, user_id: &str) -> Result<Option<User>, StorageError> {
        Ok(self
            .users
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|user| user.id() == user_id)
            .cloned())
    }

    async fn user_by_email(&self, email: &str) -> Result<Option<User>, StorageError> {
        Ok(self
            .users
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|user| {
                user.email_addresses
                    .iter()
                    .any(|email_address| email_address.email == email)
            })
            .cloned())
    }

    async fn create_user(
        &self,
        _user: CreateUser,
        _email_address: CreateEmailAddress,
    ) -> Result<User, StorageError> {
        todo!("create_user")
    }

    async fn update_user(&self, _user: UpdateUser) -> Result<User, StorageError> {
        todo!("update_user")
    }

    async fn delete_user(&self, _user_id: &str) -> Result<(), StorageError> {
        todo!("delete_user")
    }
}
