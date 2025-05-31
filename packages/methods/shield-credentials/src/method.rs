use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use shield::{
    Authentication, Method, Provider, Response, Session, SessionError, ShieldError, ShieldOptions,
    SignInCallbackRequest, SignInRequest, SignOutRequest, User,
};

use crate::{Credentials, provider::CredentialsProvider};

pub const CREDENTIALS_METHOD_ID: &str = "credentials";

pub struct CredentialsMethod<U: User, D: DeserializeOwned> {
    credentials: Arc<dyn Credentials<U, D>>,
}

impl<U: User, D: DeserializeOwned> CredentialsMethod<U, D> {
    pub fn new<C: Credentials<U, D> + 'static>(credentials: C) -> Self {
        Self {
            credentials: Arc::new(credentials),
        }
    }
}

#[async_trait]
impl<U: User + 'static, D: DeserializeOwned + 'static> Method for CredentialsMethod<U, D> {
    fn id(&self) -> String {
        CREDENTIALS_METHOD_ID.to_owned()
    }

    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        Ok(vec![Box::new(CredentialsProvider::new(
            self.credentials.clone(),
        ))])
    }

    async fn provider_by_id(
        &self,
        _provider_id: &str,
    ) -> Result<Option<Box<dyn Provider>>, ShieldError> {
        Ok(None)
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        if request.provider_id.is_some() {
            return Err(ShieldError::Validation(
                "Provider should be none.".to_owned(),
            ));
        }

        let Some(form_data) = request.form_data else {
            return Err(ShieldError::Validation("Missing form data.".to_owned()));
        };

        let data = serde_json::from_value(form_data)
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

        Ok(Response::Redirect(
            request
                .redirect_url
                .unwrap_or(options.sign_in_redirect.clone()),
        ))
    }

    async fn sign_in_callback(
        &self,
        _request: SignInCallbackRequest,
        _session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        Err(ShieldError::Validation(
            "Credentials method does not have a sign in callback.".to_owned(),
        ))
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
