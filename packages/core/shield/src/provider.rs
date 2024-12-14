use serde_json::Value;

use crate::{form::Form, shield::Shield};

pub trait Provider {
    fn id(&self) -> &'static str;

    fn subproviders(&self, shield: &Shield) -> Vec<Subprovider>;

    fn sign_in(&self, shield: &Shield, option: SignInRequest);

    fn sign_out(&self, shield: &Shield, option: SignOutRequest);
}

#[derive(Clone, Debug)]
pub struct Subprovider {
    pub subprovider_id: Option<String>,
    pub data: Option<Value>,
    pub form: Option<Form>,
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
    use crate::{shield::Shield, storage::Storage};

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

    impl Provider for TestProvider {
        fn id(&self) -> &'static str {
            self.id.unwrap_or(TEST_PROVIDER_ID)
        }

        fn subproviders(&self, _shield: &Shield) -> Vec<Subprovider> {
            vec![]
        }

        fn sign_in(&self, _shield: &Shield, _request: SignInRequest) {}

        fn sign_out(&self, _shield: &Shield, _request: SignOutRequest) {}
    }

    pub trait TestProviderStorage: Storage {}
}
