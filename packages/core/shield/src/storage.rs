use async_trait::async_trait;

use crate::{error::StorageError, user::User};

#[async_trait]
pub trait Storage<U: User>: Send + Sync {
    fn id(&self) -> String;

    async fn user_by_id(&self, user_id: &str) -> Result<Option<U>, StorageError>;

    async fn user_by_email(&self, email: &str) -> Result<Option<U>, StorageError>;
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::{error::StorageError, storage::Storage, user::tests::TestUser};

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
    }
}
