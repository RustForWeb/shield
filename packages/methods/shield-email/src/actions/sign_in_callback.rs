use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use shield::{
    Action, ActionMethod, CreateEmailAddress, CreateUser, Form, Input, InputType, InputTypeEmail,
    InputTypeSubmit, InputTypeText, InputValue, MethodSession, Request, Response, ResponseType,
    SessionAction, ShieldError, SignInCallbackAction, User, erased_action,
};

use crate::{
    options::EmailOptions, provider::EmailProvider, storage::EmailStorage, token::hash_token,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInCallbackData {
    // TODO: Validate using Fortifier.
    pub email: String,
    pub token: String,
}

pub struct EmailSignInCallbackAction<U: User> {
    options: EmailOptions,
    storage: Arc<dyn EmailStorage<U>>,
}

impl<U: User> EmailSignInCallbackAction<U> {
    pub fn new(options: EmailOptions, storage: Arc<dyn EmailStorage<U>>) -> Self {
        Self { options, storage }
    }
}

#[async_trait]
impl<U: User + 'static> Action<EmailProvider, ()> for EmailSignInCallbackAction<U> {
    fn id(&self) -> String {
        SignInCallbackAction::id()
    }

    fn name(&self) -> String {
        SignInCallbackAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in callback for email"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in callback for email."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    fn condition(
        &self,
        provider: &EmailProvider,
        session: &MethodSession<()>,
    ) -> Result<bool, ShieldError> {
        SignInCallbackAction::condition(provider, session)
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
                    value: Some(InputValue::Query {
                        key: "email".to_owned(),
                    }),
                    addon_start: None,
                    addon_end: None,
                },
                Input {
                    name: "token".to_owned(),
                    label: Some("Token".to_owned()),
                    r#type: InputType::Text(InputTypeText {
                        placeholder: Some("Token".to_owned()),
                        required: Some(true),
                        ..Default::default()
                    }),
                    value: Some(InputValue::Query {
                        key: "token".to_owned(),
                    }),
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
        let data = serde_json::from_value::<SignInCallbackData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let email_auth_token = self
            .storage
            .email_auth_token(
                &data.email.to_lowercase(),
                &hash_token(&data.token, &self.options.secret),
            )
            .await?
            .ok_or_else(|| {
                ShieldError::Validation("Email authentication token not found.".to_owned())
            })?;

        self.storage
            .delete_email_auth_token(&email_auth_token.id)
            .await?;

        let user = match self.storage.user_by_email(&email_auth_token.email).await? {
            Some(user) => user,
            None => {
                self.storage
                    .create_user(
                        CreateUser { name: None },
                        CreateEmailAddress {
                            email: email_auth_token.email,
                            is_primary: true,
                            is_verified: true,
                            verification_token: None,
                            verification_token_expired_at: None,
                            verified_at: Some(Utc::now().into()),
                        },
                    )
                    .await?
            }
        };

        Ok(Response::new(ResponseType::Redirect(
            self.options.sign_in_redirect.clone(),
        ))
        .session_action(SessionAction::authenticate(user)))
    }
}

erased_action!(EmailSignInCallbackAction, <U: User>);
