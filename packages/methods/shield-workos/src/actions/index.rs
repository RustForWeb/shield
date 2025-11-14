use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shield::{
    Action, ActionMethod, Form, Input, InputType, InputTypeEmail, InputTypeHidden, InputTypeSubmit,
    InputValue, MethodSession, Request, Response, ResponseType, ShieldError, SignInAction,
    SignUpAction, erased_action,
};
use workos::{
    PaginationParams,
    sso::{ConnectionId, ListConnections, ListConnectionsParams},
    user_management::{
        ConnectionSelector, GetAuthorizationUrl, GetAuthorizationUrlParams, ListUsers,
        ListUsersParams, OauthProvider, Provider,
    },
};

use crate::{client::WorkosClient, options::WorkosOptions, provider::WorkosProvider};

const ACTION_ID: &str = "index";
const ACTION_NAME: &str = "Welcome";

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum IndexData {
    Email { email: String },
    Oauth { oauth_provider: OauthProvider },
    Sso { connection_id: ConnectionId },
}

pub struct WorkosIndexAction {
    options: WorkosOptions,
    client: Arc<WorkosClient>,
}

impl WorkosIndexAction {
    pub fn new(options: WorkosOptions, client: Arc<WorkosClient>) -> Self {
        Self { options, client }
    }
}

#[async_trait]
impl Action<WorkosProvider, ()> for WorkosIndexAction {
    fn id(&self) -> String {
        ACTION_ID.to_owned()
    }

    fn name(&self) -> String {
        ACTION_NAME.to_owned()
    }

    fn openapi_summary(&self) -> &'static str {
        "Index with WorkOS"
    }

    fn openapi_description(&self) -> &'static str {
        "Index with WorkOS."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    async fn forms(&self, _provider: WorkosProvider) -> Result<Vec<Form>, ShieldError> {
        let connections = self
            .client
            .sso()
            .list_connections(&ListConnectionsParams {
                pagination: PaginationParams {
                    limit: Some(100),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .expect("TODO: handle error");

        Ok([Form {
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
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: "Continue".to_owned(),
                    }),
                },
            ],
        }]
        .into_iter()
        .chain(
            self.options
                .oauth_providers
                .iter()
                .map(|oauth_provider| Form {
                    inputs: vec![
                        Input {
                            name: "oauthProvider".to_owned(),
                            label: None,
                            r#type: InputType::Hidden(InputTypeHidden {
                                required: Some(true),
                                ..Default::default()
                            }),
                            value: Some(InputValue::String {
                                value: oauth_provider.to_string(),
                            }),
                        },
                        Input {
                            name: "submit".to_owned(),
                            label: None,
                            r#type: InputType::Submit(InputTypeSubmit::default()),
                            value: Some(InputValue::String {
                                value: format!(
                                    "Continue with {}",
                                    match oauth_provider {
                                        OauthProvider::AppleOAuth => "Apple",
                                        OauthProvider::GithubOAuth => "GitHub",
                                        OauthProvider::GoogleOAuth => "Google",
                                        OauthProvider::MicrosoftOAuth => "Microsoft",
                                    }
                                )
                                .to_owned(),
                            }),
                        },
                    ],
                }),
        )
        .chain(connections.data.into_iter().map(|connection| Form {
            inputs: vec![
                Input {
                    name: "connectionId".to_owned(),
                    label: None,
                    r#type: InputType::Hidden(InputTypeHidden {
                        required: Some(true),
                        ..Default::default()
                    }),
                    value: Some(InputValue::String {
                        value: connection.id.to_string(),
                    }),
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: format!("Continue with {}", connection.name).to_owned(),
                    }),
                },
            ],
        }))
        .collect())
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: &MethodSession<()>,
        request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: Check email address and redirect to sign-in/sign-up action with prefilled email address.
        // TODO: Only check if enabled in options.

        let data = serde_json::from_value::<IndexData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        match data {
            IndexData::Email { email } => {
                let users = self
                    .client
                    .user_management()
                    .list_users(&ListUsersParams {
                        pagination: PaginationParams {
                            limit: Some(1),
                            ..Default::default()
                        },
                        email: Some(&email),
                        ..Default::default()
                    })
                    .await
                    .expect("TODO: handle error");

                // TODO: Include email address as state.
                if users.data.is_empty() {
                    Ok(Response::new(ResponseType::RedirectToAction {
                        action_id: SignUpAction::id(),
                    }))
                } else {
                    Ok(Response::new(ResponseType::RedirectToAction {
                        action_id: SignInAction::id(),
                    }))
                }
            }
            IndexData::Oauth { oauth_provider } => {
                let authorization_url = self
                    .client
                    .user_management()
                    .get_authorization_url(&GetAuthorizationUrlParams {
                        client_id: &self.client.client_id(),
                        redirect_uri: &self.options.redirect_url,
                        connection_selector: ConnectionSelector::Provider(&Provider::Oauth(
                            oauth_provider,
                        )),
                        // TODO: State and code challenge.
                        state: None,
                        code_challenge: None,
                        login_hint: None,
                        domain_hint: None,
                    })
                    .expect("TODO: handle error");

                Ok(Response::new(ResponseType::Redirect(
                    authorization_url.to_string(),
                )))
            }
            IndexData::Sso { connection_id } => {
                let authorization_url = self
                    .client
                    .user_management()
                    .get_authorization_url(&GetAuthorizationUrlParams {
                        client_id: &self.client.client_id(),
                        redirect_uri: &self.options.redirect_url,
                        connection_selector: ConnectionSelector::Connection(&connection_id),
                        // TODO: State and code challenge.
                        state: None,
                        code_challenge: None,
                        login_hint: None,
                        domain_hint: None,
                    })
                    .expect("TODO: handle error");

                Ok(Response::new(ResponseType::Redirect(
                    authorization_url.to_string(),
                )))
            }
        }
    }
}

erased_action!(WorkosIndexAction);
