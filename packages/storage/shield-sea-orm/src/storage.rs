use async_trait::async_trait;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use shield::{CreateEmailAddress, CreateUser, Storage, StorageError, UpdateUser};

use crate::entities::{email_address, prelude::User, user};

pub const SEA_ORM_STORAGE_ID: &str = "sea-orm";

impl shield::User for user::Model {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct SeaOrmStorage {
    pub(crate) database: DatabaseConnection,
}

impl SeaOrmStorage {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub(crate) fn parse_uuid(uuid: &str) -> Result<Uuid, StorageError> {
        Uuid::parse_str(uuid).map_err(|err| StorageError::Validation(err.to_string()))
    }
}

#[async_trait]
impl Storage<user::Model> for SeaOrmStorage {
    fn id(&self) -> String {
        SEA_ORM_STORAGE_ID.to_owned()
    }

    async fn user_by_id(&self, user_id: &str) -> Result<Option<user::Model>, StorageError> {
        User::find_by_id(Self::parse_uuid(user_id)?)
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
    }

    async fn user_by_email(&self, email: &str) -> Result<Option<user::Model>, StorageError> {
        User::find()
            .left_join(email_address::Entity)
            .filter(email_address::Column::Email.eq(email))
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
    }

    async fn create_user(
        &self,
        user: CreateUser,
        email_address: CreateEmailAddress,
    ) -> Result<user::Model, StorageError> {
        // TODO: transaction

        let active_model = user::ActiveModel {
            name: ActiveValue::Set(user.name.unwrap_or_default()),
            ..Default::default()
        };

        let user = active_model
            .insert(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))?;

        let active_model = email_address::ActiveModel {
            email: ActiveValue::Set(email_address.email),
            is_primary: ActiveValue::Set(email_address.is_primary),
            is_verified: ActiveValue::Set(email_address.is_verified),
            verification_token: ActiveValue::Set(email_address.verification_token),
            verification_token_expired_at: ActiveValue::Set(
                email_address.verification_token_expired_at,
            ),
            verified_at: ActiveValue::Set(email_address.verified_at),
            user_id: ActiveValue::Set(user.id),
            ..Default::default()
        };

        active_model
            .insert(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))?;

        Ok(user)
    }

    async fn update_user(&self, user: UpdateUser) -> Result<user::Model, StorageError> {
        let mut active_model: user::ActiveModel = user::Entity::find()
            .filter(user::Column::Id.eq(Self::parse_uuid(&user.id)?))
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))?
            .ok_or_else(|| StorageError::NotFound("User".to_owned(), user.id))?
            .into();

        if let Some(Some(name)) = user.name {
            active_model.name = ActiveValue::Set(name);
        }

        active_model
            .update(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), StorageError> {
        user::Entity::delete_by_id(Self::parse_uuid(user_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}
