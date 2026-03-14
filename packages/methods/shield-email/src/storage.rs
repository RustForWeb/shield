use async_trait::async_trait;

use shield::{Storage, StorageError, User};

use crate::token::{CreateEmailAuthToken, EmailAuthToken};

#[async_trait]
pub trait EmailStorage<U: User>: Storage<U> + Sync {
    async fn email_auth_token(
        &self,
        email: &str,
        token: &str,
    ) -> Result<Option<EmailAuthToken>, StorageError>;

    async fn create_email_auth_token(
        &self,
        email_auth_token: CreateEmailAuthToken,
    ) -> Result<EmailAuthToken, StorageError>;

    async fn delete_email_auth_token(&self, email_auth_token_id: &str) -> Result<(), StorageError>;

    async fn delete_expired_email_auth_tokens(&self) -> Result<(), StorageError>;
}
