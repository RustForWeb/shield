use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use shield::StorageError;
use shield_email::{CreateEmailAuthToken, EmailAuthToken, EmailStorage};

use crate::{entities::email_auth_token, storage::SeaOrmStorage, user::User};

#[async_trait]
impl EmailStorage<User> for SeaOrmStorage {
    async fn email_auth_token(
        &self,
        email: &str,
        token: &str,
    ) -> Result<Option<EmailAuthToken>, StorageError> {
        email_auth_token::Entity::find()
            .filter(email_auth_token::Column::Email.eq(email))
            .filter(email_auth_token::Column::Token.eq(token))
            .filter(email_auth_token::Column::ExpiredAt.gt(Utc::now()))
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|email_auth_token| email_auth_token.map(EmailAuthToken::from))
    }

    async fn create_email_auth_token(
        &self,
        email_auth_token: CreateEmailAuthToken,
    ) -> Result<EmailAuthToken, StorageError> {
        let active_model = email_auth_token::ActiveModel {
            email: ActiveValue::Set(email_auth_token.email),
            token: ActiveValue::Set(email_auth_token.token),
            expired_at: ActiveValue::Set(email_auth_token.expired_at),
            ..Default::default()
        };

        active_model
            .insert(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(EmailAuthToken::from)
    }

    async fn delete_email_auth_token(&self, email_auth_token_id: &str) -> Result<(), StorageError> {
        email_auth_token::Entity::delete_by_id(Self::parse_uuid(email_auth_token_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }

    async fn delete_expired_email_auth_tokens(&self) -> Result<(), StorageError> {
        email_auth_token::Entity::delete_many()
            .filter(email_auth_token::Column::ExpiredAt.lte(Utc::now()))
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}

impl From<email_auth_token::Model> for EmailAuthToken {
    fn from(value: email_auth_token::Model) -> Self {
        EmailAuthToken {
            id: value.id.to_string(),
            email: value.email,
            token: value.token,
            expired_at: value.expired_at,
        }
    }
}
