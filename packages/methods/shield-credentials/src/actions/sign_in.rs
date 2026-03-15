use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use shield::{
    Form, MethodAction, MethodSession, Request, RequestMethod, Response, ResponseType,
    SessionAction, ShieldError, SignInAction, User, erased_method_action,
};

use crate::{credentials::Credentials, provider::CredentialsProvider};

pub struct CredentialsSignInAction<U: User, D: DeserializeOwned> {
    credentials: Arc<dyn Credentials<U, D>>,
}

impl<U: User, D: DeserializeOwned> CredentialsSignInAction<U, D> {
    pub fn new(credentials: Arc<dyn Credentials<U, D>>) -> Self {
        Self { credentials }
    }
}

#[async_trait]
impl<U: User + 'static, D: DeserializeOwned + 'static> MethodAction<CredentialsProvider, ()>
    for CredentialsSignInAction<U, D>
{
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in with credentials"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in with credentials."
    }

    fn method(&self) -> RequestMethod {
        RequestMethod::Post
    }

    async fn forms(&self, _provider: CredentialsProvider) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![self.credentials.form()])
    }

    async fn call(
        &self,
        provider: CredentialsProvider,
        _session: &MethodSession<()>,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let data = serde_json::from_value(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let user = self.credentials.sign_in(data).await?;

        Ok(Response::new(ResponseType::Default)
            .session_action(SessionAction::authenticate(&provider, user)))
    }
}

erased_method_action!(CredentialsSignInAction, <U: User, D: DeserializeOwned>);
