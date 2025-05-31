use async_trait::async_trait;

use crate::{
    error::ShieldError,
    options::ShieldOptions,
    provider::Provider,
    request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
    response::Response,
    session::Session,
};

#[async_trait]
pub trait Method: Send + Sync {
    fn id(&self) -> String;

    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError>;

    async fn provider_by_id(
        &self,
        provider_id: &str,
    ) -> Result<Option<Box<dyn Provider>>, ShieldError>;

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Response, ShieldError>;

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Response, ShieldError>;

    async fn sign_out(
        &self,
        request: SignOutRequest,
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Option<Response>, ShieldError>;
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::{
        ShieldOptions,
        error::ShieldError,
        provider::Provider,
        request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
        response::Response,
        session::Session,
    };

    use super::Method;

    pub const TEST_METHOD_ID: &str = "test";

    #[derive(Default)]
    pub struct TestMethod {
        id: Option<&'static str>,
    }

    impl TestMethod {
        pub fn with_id(mut self, id: &'static str) -> Self {
            self.id = Some(id);
            self
        }
    }

    #[async_trait]
    impl Method for TestMethod {
        fn id(&self) -> String {
            self.id.unwrap_or(TEST_METHOD_ID).to_owned()
        }

        async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
            Ok(vec![])
        }

        async fn provider_by_id(
            &self,
            _provider_id: &str,
        ) -> Result<Option<Box<dyn Provider>>, ShieldError> {
            Ok(None)
        }

        async fn sign_in(
            &self,
            _request: SignInRequest,
            _session: Session,
            _options: &ShieldOptions,
        ) -> Result<Response, ShieldError> {
            todo!("redirect back?")
        }

        async fn sign_in_callback(
            &self,
            _request: SignInCallbackRequest,
            _session: Session,
            _options: &ShieldOptions,
        ) -> Result<Response, ShieldError> {
            todo!("redirect back?")
        }

        async fn sign_out(
            &self,
            _request: SignOutRequest,
            _session: Session,
            _options: &ShieldOptions,
        ) -> Result<Option<Response>, ShieldError> {
            Ok(None)
        }
    }
}
