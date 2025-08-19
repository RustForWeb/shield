use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shield::{
    Action, Form, Input, InputType, InputTypeEmail, InputTypeHidden, InputTypeSubmit, Request,
    Response, Session, ShieldError, erased_action,
};
use tracing::info;
use workos_sdk::{
    PaginationParams, WorkOs,
    sso::{ConnectionId, ListConnections, ListConnectionsParams},
    user_management::{ListUsers, ListUsersParams, OauthProvider},
};

use crate::{WorkosOptions, provider::WorkosProvider};

// TODO: Make a special case for an index action reachable at the `/auth` root URL.

const ACTION_ID: &str = "index";
const ACTION_NAME: &str = "Index";

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum IndexData {
    Email {
        // TODO: Dioxus records multiple values per field, but most of the time only a single value is expected.
        email: Vec<String>,
    },
    Oauth {
        // TODO: See above.
        oauth_provider: Vec<OauthProvider>,
    },
    Sso {
        // TODO: See above.
        connection_id: Vec<ConnectionId>,
    },
}

pub struct WorkosIndexAction {
    options: WorkosOptions,
    client: Arc<WorkOs>,
}

impl WorkosIndexAction {
    pub fn new(options: WorkosOptions, client: Arc<WorkOs>) -> Self {
        Self { options, client }
    }
}

#[async_trait]
impl Action<WorkosProvider> for WorkosIndexAction {
    fn id(&self) -> String {
        ACTION_ID.to_owned()
    }

    fn name(&self) -> String {
        ACTION_NAME.to_owned()
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

        info!("{connections:#?}");

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
                    value: Some("Continue".to_owned()),
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
                            value: Some(oauth_provider.to_string()),
                        },
                        Input {
                            name: "submit".to_owned(),
                            label: None,
                            r#type: InputType::Submit(InputTypeSubmit::default()),
                            value: Some(
                                format!(
                                    "Continue with {}",
                                    match oauth_provider {
                                        OauthProvider::AppleOAuth => "Apple",
                                        OauthProvider::GithubOAuth => "GitHub",
                                        OauthProvider::GoogleOAuth => "Google",
                                        OauthProvider::MicrosoftOAuth => "Microsoft",
                                    }
                                )
                                .to_owned(),
                            ),
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
                    value: Some(connection.id.to_string()),
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(format!("Continue with {}", connection.name).to_owned()),
                },
            ],
        }))
        .collect())
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: Session,
        request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: Check email address and redirect to sign-in/sign-up action with prefilled email address.
        // TODO: Only check if enabled in options.

        let data = serde_json::from_value::<IndexData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        match data {
            IndexData::Email { email } => {
                info!("email: {email:#?}");

                let users = self
                    .client
                    .user_management()
                    .list_users(&ListUsersParams {
                        pagination: PaginationParams {
                            limit: Some(1),
                            ..Default::default()
                        },
                        // TODO: Remove [0] once email is a single value.
                        email: Some(&email[0]),
                        ..Default::default()
                    })
                    .await
                    .expect("TODO: handle error");

                info!("{users:#?}");

                if users.data.is_empty() {
                    // TODO: Redirect to sign up action.
                } else {
                    // TODO: Redirect to sign in action.
                }
            }
            IndexData::Oauth { oauth_provider } => {
                info!("oauth {oauth_provider:#?}");

                // TODO: Add client ID to method.
                // self.client
                //     .user_management()
                //     .get_authorization_url(&GetAuthorizationUrlParams {
                //         client_id: todo!(),
                //         redirect_uri: todo!(),
                //         connection_selector: todo!(),
                //         state: todo!(),
                //         code_challenge: todo!(),
                //         login_hint: todo!(),
                //         domain_hint: todo!(),
                //     })
            }
            IndexData::Sso { connection_id } => {
                info!("sso {connection_id:#?}");
            }
        }

        Ok(Response::Default)
    }
}

erased_action!(WorkosIndexAction);
