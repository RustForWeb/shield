use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use shield::{
    Action, Authentication, Form, Request, Response, Session, SessionError, ShieldError,
    SignInAction, User, erased_action,
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
impl<U: User + 'static, D: DeserializeOwned + 'static> Action<CredentialsProvider>
    for CredentialsSignInAction<U, D>
{
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    async fn forms(&self, _provider: CredentialsProvider) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![self.credentials.form()])
    }

    async fn call(
        &self,
        _provider: CredentialsProvider,
        session: Session,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let data = serde_json::from_value(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let user = self.credentials.sign_in(data).await?;

        session.renew().await?;

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = Some(Authentication {
                method_id: self.id(),
                provider_id: None,
                user_id: user.id(),
            });
        }

        Ok(Response::Default)
    }
}

erased_action!(CredentialsSignInAction, <U: User, D: DeserializeOwned>);
