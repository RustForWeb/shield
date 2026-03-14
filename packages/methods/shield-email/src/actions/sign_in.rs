use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use rand::distr::{Alphanumeric, SampleString};
use serde::Deserialize;
use shield::{
    Action, ActionMethod, Form, Input, InputType, InputTypeEmail, InputTypeSubmit, InputValue,
    MethodSession, Request, Response, ResponseType, SessionAction, ShieldError, SignInAction, User,
    erased_action,
};

use crate::{
    options::EmailOptions,
    provider::EmailProvider,
    storage::EmailStorage,
    token::{CreateEmailAuthToken, hash_token},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInData {
    // TODO: Validate using Fortifier.
    pub email: String,
}

pub struct EmailSignInAction<U: User> {
    options: EmailOptions,
    storage: Arc<dyn EmailStorage<U>>,
}

impl<U: User> EmailSignInAction<U> {
    pub fn new(options: EmailOptions, storage: Arc<dyn EmailStorage<U>>) -> Self {
        Self { options, storage }
    }
}

#[async_trait]
impl<U: User + 'static> Action<EmailProvider, ()> for EmailSignInAction<U> {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in with email"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in with email."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    async fn forms(&self, _provider: EmailProvider) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![Form {
            inputs: vec![
                Input {
                    name: "email".to_owned(),
                    label: Some("Email address".to_owned()),
                    r#type: InputType::Email(InputTypeEmail {
                        autocomplete: Some("email".to_owned()),
                        placeholder: Some("Email address".to_owned()),
                        required: Some(true),
                        ..Default::default()
                    }),
                    value: None,
                    addon_start: None,
                    addon_end: None,
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: "Sign in with email".to_owned(),
                    }),
                    addon_start: None,
                    addon_end: None,
                },
            ],
        }])
    }

    async fn call(
        &self,
        _provider: EmailProvider,
        _session: &MethodSession<()>,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let data = serde_json::from_value::<SignInData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let token = Alphanumeric.sample_string(&mut rand::rng(), 32);
        let expires_at = Utc::now() + self.options.expires_in;

        let email_auth_token = self
            .storage
            .create_email_auth_token(CreateEmailAuthToken {
                email: data.email.to_lowercase(),
                token: hash_token(&token, &self.options.secret),
                expired_at: expires_at.into(),
            })
            .await?;

        self.options
            .sender
            .send(
                &email_auth_token.email,
                &email_auth_token.token,
                email_auth_token.expired_at,
            )
            .await?;

        Ok(Response::new(ResponseType::Default).session_action(SessionAction::unauthenticate()))
    }
}

erased_action!(EmailSignInAction, <U: User>);
