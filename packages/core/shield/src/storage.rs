use async_trait::async_trait;

use crate::{
    error::StorageError,
    user::{CreateEmailAddress, CreateUser, UpdateUser, User},
};

#[async_trait]
pub trait Storage<U: User>: Send + Sync {
    fn id(&self) -> String;

    async fn user_by_id(&self, user_id: &str) -> Result<Option<U>, StorageError>;

    async fn user_by_email(&self, email: &str) -> Result<Option<U>, StorageError>;

    async fn create_user(
        &self,
        user: CreateUser,
        email_address: CreateEmailAddress,
    ) -> Result<U, StorageError>;

    async fn update_user(&self, user: UpdateUser) -> Result<U, StorageError>;

    async fn delete_user(&self, user_id: &str) -> Result<(), StorageError>;

    // TODO: create, update, delete email address
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::{
        error::StorageError,
        storage::Storage,
        user::{tests::TestUser, CreateEmailAddress, CreateUser, UpdateUser},
    };

    pub const TEST_STORAGE_ID: &str = "test";

    #[derive(Default)]
    pub struct TestStorage {}

    #[async_trait]
    impl Storage<TestUser> for TestStorage {
        fn id(&self) -> String {
            TEST_STORAGE_ID.to_owned()
        }

        async fn user_by_id(&self, _user_id: &str) -> Result<Option<TestUser>, StorageError> {
            todo!("user_by_id")
        }

        async fn user_by_email(&self, _email: &str) -> Result<Option<TestUser>, StorageError> {
            todo!("user_by_email")
        }

        async fn create_user(
            &self,
            _user: CreateUser,
            _email_address: CreateEmailAddress,
        ) -> Result<TestUser, StorageError> {
            todo!("create_user")
        }

        async fn update_user(&self, _user: UpdateUser) -> Result<TestUser, StorageError> {
            todo!("update_user")
        }

        async fn delete_user(&self, _user_id: &str) -> Result<(), StorageError> {
            todo!("delete_user")
        }
    }
}
