use async_trait::async_trait;
pub use serde_json::Value;

use crate::{form::Form, StorageError};

#[async_trait]
pub trait Provider {
    fn id(&self) -> &'static str;

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError>;

    async fn sign_in(&self, option: SignInRequest);

    async fn sign_out(&self, option: SignOutRequest);
}

pub trait Subprovider {
    fn id(&self) -> Option<String>;

    fn data(&self) -> Option<Value>;

    fn form(&self) -> Option<Form>;
}

#[derive(Clone, Debug)]
pub struct SignInRequest {
    pub subprovider_id: Option<String>,
    pub data: Option<Value>,
    pub form_data: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct SignOutRequest {
    pub subprovider_id: Option<String>,
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::StorageError;

    use super::{Provider, SignInRequest, SignOutRequest, Subprovider};

    pub const TEST_PROVIDER_ID: &str = "test";

    #[derive(Default)]
    pub struct TestProvider {
        id: Option<&'static str>,
    }

    impl TestProvider {
        pub fn with_id(mut self, id: &'static str) -> Self {
            self.id = Some(id);
            self
        }
    }

    #[async_trait]
    impl Provider for TestProvider {
        fn id(&self) -> &'static str {
            self.id.unwrap_or(TEST_PROVIDER_ID)
        }

        async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError> {
            Ok(vec![])
        }

        async fn sign_in(&self, _request: SignInRequest) {}

        async fn sign_out(&self, _request: SignOutRequest) {}
    }
}
