use async_trait::async_trait;
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shield::{Storage, StorageError};

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
}
