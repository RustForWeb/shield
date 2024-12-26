use async_trait::async_trait;
use shield::{Storage, StorageError};

use crate::entities::user;

pub const SEA_ORM_STORAGE_ID: &str = "sea-orm";

impl shield::User for user::Model {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct SeaOrmStorage {}

#[async_trait]
impl Storage<user::Model> for SeaOrmStorage {
    fn id(&self) -> String {
        SEA_ORM_STORAGE_ID.to_owned()
    }

    async fn user_by_id(&self, _user_id: &str) -> Result<Option<user::Model>, StorageError> {
        todo!("user_by_id")
    }

    async fn user_by_email(&self, _email: &str) -> Result<Option<user::Model>, StorageError> {
        todo!("user_by_id")
    }
}
