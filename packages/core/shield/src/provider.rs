use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::ShieldError,
    form::Form,
    request::{SignInRequest, SignOutRequest},
};

#[async_trait]
pub trait Provider: Send + Sync {
    fn id(&self) -> String;

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError>;

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, ShieldError>;

    async fn sign_in(&self, request: SignInRequest) -> Result<(), ShieldError>;

    async fn sign_out(&self, request: SignOutRequest) -> Result<(), ShieldError>;
}

pub trait Subprovider: Send + Sync {
    fn provider_id(&self) -> String;

    fn id(&self) -> Option<String>;

    fn name(&self) -> String;

    fn form(&self) -> Option<Form>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubproviderVisualisation {
    pub key: String,
    pub provider_id: String,
    pub subprovider_id: Option<String>,
    pub name: String,
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use super::{Provider, ShieldError, SignInRequest, SignOutRequest, Subprovider};

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
        fn id(&self) -> String {
            self.id.unwrap_or(TEST_PROVIDER_ID).to_owned()
        }

        async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
            Ok(vec![])
        }

        async fn subprovider_by_id(
            &self,
            _subprovider_id: &str,
        ) -> Result<Option<Box<dyn Subprovider>>, ShieldError> {
            Ok(None)
        }

        async fn sign_in(&self, _request: SignInRequest) -> Result<(), ShieldError> {
            Ok(())
        }

        async fn sign_out(&self, _request: SignOutRequest) -> Result<(), ShieldError> {
            Ok(())
        }
    }
}
