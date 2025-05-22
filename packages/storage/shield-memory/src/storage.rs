use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::{
    CreateEmailAddress, CreateUser, EmailAddress, Storage, StorageError, UpdateUser, User as _,
};
use uuid::Uuid;

use crate::user::User;

pub const MEMORY_STORAGE_ID: &str = "memory";

#[derive(Clone, Debug, Default)]
pub struct MemoryStorage {
    pub(crate) users: Arc<Mutex<Vec<User>>>,
    #[cfg(feature = "method-oidc")]
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
        user: CreateUser,
        email_address: CreateEmailAddress,
    ) -> Result<User, StorageError> {
        let user_id = Uuid::new_v4().to_string();

        let user = User {
            id: user_id.clone(),
            name: user.name,
            email_addresses: vec![EmailAddress {
                id: Uuid::new_v4().to_string(),
                email: email_address.email,
                is_primary: email_address.is_primary,
                is_verified: email_address.is_verified,
                verification_token: email_address.verification_token,
                verification_token_expired_at: email_address.verification_token_expired_at,
                verified_at: email_address.verified_at,
                user_id,
            }],
        };

        self.users
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .push(user.clone());

        Ok(user)
    }

    async fn update_user(&self, user: UpdateUser) -> Result<User, StorageError> {
        let mut users = self
            .users
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?;

        let user_mut = users
            .iter_mut()
            .find(|u| u.id() == user.id)
            .ok_or_else(|| StorageError::NotFound("User".to_owned(), user.id.clone()))?;

        if let Some(name) = user.name {
            user_mut.name = name;
        }

        Ok(user_mut.clone())
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), StorageError> {
        self.users
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .retain(|user| user.id != user_id);

        Ok(())
    }
}
