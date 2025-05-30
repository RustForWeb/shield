use std::ops::Deref;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, ModelTrait, prelude::Uuid};
use serde::Serialize;
use shield::{EmailAddress, StorageError};

#[cfg(feature = "entity")]
use crate::entities::entity;
use crate::entities::{email_address, user};

#[derive(Clone, Debug)]
pub struct User {
    database: DatabaseConnection,
    user: user::Model,
    #[cfg(feature = "entity")]
    entity: entity::Model,
}

impl User {
    pub(crate) fn new(
        database: DatabaseConnection,
        user: user::Model,
        #[cfg(feature = "entity")] entity: entity::Model,
    ) -> Self {
        Self {
            database,
            user,
            #[cfg(feature = "entity")]
            entity,
        }
    }

    #[cfg(feature = "entity")]
    pub fn entity(&self) -> &entity::Model {
        &self.entity
    }
}

impl Deref for User {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Additional {
    #[cfg(feature = "entity")]
    entity_id: String,
}

#[async_trait]
impl shield::User for User {
    fn id(&self) -> String {
        self.user.id.to_string()
    }

    fn name(&self) -> Option<String> {
        #[cfg(feature = "entity")]
        {
            Some(self.entity.name.clone())
        }

        #[cfg(not(feature = "entity"))]
        {
            Some(self.user.name.clone())
        }
    }

    async fn email_addresses(&self) -> Result<Vec<EmailAddress>, StorageError> {
        #[cfg(feature = "entity")]
        {
            self.entity
                .find_related(email_address::Entity)
                .all(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
                .map(|email_addresses| {
                    email_addresses
                        .into_iter()
                        .map(|email_address| {
                            EmailAddress::from(EmailAddressWithUserId(email_address, self.user.id))
                        })
                        .collect()
                })
        }

        #[cfg(not(feature = "entity"))]
        {
            self.user
                .find_related(email_address::Entity)
                .all(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
                .map(|email_addresses| {
                    email_addresses
                        .into_iter()
                        .map(EmailAddress::from)
                        .collect()
                })
        }
    }

    fn additional(&self) -> Option<impl Serialize> {
        Some(Additional {
            #[cfg(feature = "entity")]
            entity_id: self.user.entity_id.to_string(),
        })
    }
}

#[cfg(not(feature = "entity"))]
impl From<email_address::Model> for EmailAddress {
    fn from(value: email_address::Model) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            is_primary: value.is_primary,
            is_verified: value.is_verified,
            verification_token: value.verification_token,
            verification_token_expired_at: value.verification_token_expired_at,
            verified_at: value.verified_at,
            user_id: value.user_id.to_string(),
        }
    }
}

struct EmailAddressWithUserId(email_address::Model, Uuid);

impl From<EmailAddressWithUserId> for EmailAddress {
    fn from(EmailAddressWithUserId(value, user_id): EmailAddressWithUserId) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            is_primary: value.is_primary,
            is_verified: value.is_verified,
            verification_token: value.verification_token,
            verification_token_expired_at: value.verification_token_expired_at,
            verified_at: value.verified_at,
            user_id: user_id.to_string(),
        }
    }
}
