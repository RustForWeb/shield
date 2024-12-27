use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use shield::{Storage, StorageError};

use crate::user::User;

pub const MEMORY_STORAGE_ID: &str = "memory";

#[derive(Clone, Debug, Default)]
pub struct MemoryStorage {
    pub(crate) users: Arc<Mutex<HashMap<String, User>>>,
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
            .get(user_id)
            .cloned())
    }

    async fn user_by_email(&self, _email: &str) -> Result<Option<User>, StorageError> {
        todo!("user_by_email")
    }
}
