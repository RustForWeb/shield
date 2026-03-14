use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use shield::StorageError;
use shield_email::{CreateEmailAuthToken, EmailAuthToken, EmailStorage};
use uuid::Uuid;

use crate::{storage::MemoryStorage, user::User};

#[derive(Clone, Debug, Default)]
pub struct EmailMemoryStorage {
    email_auth_tokens: Arc<Mutex<Vec<EmailAuthToken>>>,
}

#[async_trait]
impl EmailStorage<User> for MemoryStorage {
    async fn email_auth_token(
        &self,
        email: &str,
        token: &str,
    ) -> Result<Option<EmailAuthToken>, StorageError> {
        Ok(self
            .email
            .email_auth_tokens
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .iter()
            .find(|email_auth_token| {
                email_auth_token.email == email
                    && email_auth_token.token == token
                    && email_auth_token.expired_at < Utc::now()
            })
            .cloned())
    }

    async fn create_email_auth_token(
        &self,
        email_auth_token: CreateEmailAuthToken,
    ) -> Result<EmailAuthToken, StorageError> {
        let email_auth_token = EmailAuthToken {
            id: Uuid::new_v4().to_string(),
            email: email_auth_token.email,
            token: email_auth_token.token,
            expired_at: email_auth_token.expired_at,
        };

        self.email
            .email_auth_tokens
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .push(email_auth_token.clone());

        Ok(email_auth_token)
    }

    async fn delete_email_auth_token(&self, email_auth_token_id: &str) -> Result<(), StorageError> {
        self.email
            .email_auth_tokens
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .retain(|email_auth_token| email_auth_token.id != email_auth_token_id);

        Ok(())
    }

    async fn delete_expired_email_auth_tokens(&self) -> Result<(), StorageError> {
        let now = Utc::now();

        self.email
            .email_auth_tokens
            .lock()
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .retain(|email_auth_token| email_auth_token.expired_at > now);

        Ok(())
    }
}
