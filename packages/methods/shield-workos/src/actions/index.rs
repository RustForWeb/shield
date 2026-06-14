use async_trait::async_trait;
use serde::Deserialize;
use shield::{
    Form, Input, InputType, InputTypeEmail, InputTypeHidden, InputTypeSubmit, InputValue,
    MethodAction, MethodSession, Request, RequestMethod, Response, ResponseType, ShieldError,
    SignInAction, SignUpAction, erased_method_action,
};
use workos::{
    Client, UserManagementAuthenticationProvider,
    sso::ListConnectionsParams,
    user_management::{GetAuthorizationUrlParams, ListUsersParams},
};

use crate::{options::WorkosOptions, provider::WorkosProvider};

const ACTION_ID: &str = "index";
const ACTION_NAME: &str = "Welcome";

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum IndexData {
    Email {
        email: String,
    },
    Oauth {
        oauth_provider: UserManagementAuthenticationProvider,
    },
    Sso {
        connection_id: String,
    },
}

pub struct WorkosIndexAction {
    options: WorkosOptions,
    client: Client,
}

impl WorkosIndexAction {
    pub fn new(options: WorkosOptions, client: Client) -> Self {
        Self { options, client }
    }
}

#[async_trait]
impl MethodAction<WorkosProvider, ()> for WorkosIndexAction {
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

    fn method(&self) -> RequestMethod {
        RequestMethod::Post
    }

    async fn forms(&self, _provider: WorkosProvider) -> Result<Vec<Form>, ShieldError> {
        let connections = self
            .client
            .sso()
            .list_connections(ListConnectionsParams {
                limit: Some(100),
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
                    addon_start: None,
                    addon_end: None,
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: "Continue".to_owned(),
                    }),
                    addon_start: None,
                    addon_end: None,
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
                            addon_start: None,
                            addon_end: None,
                        },
                        Input {
                            name: "submit".to_owned(),
                            label: None,
                            r#type: InputType::Submit(InputTypeSubmit::default()),
                            value: Some(InputValue::String {
                                value: format!(
                                    "Continue with {}",
                                    match oauth_provider {
                                        UserManagementAuthenticationProvider::Authkit => "Authkit".to_owned(),
                                        UserManagementAuthenticationProvider::AppleOAuth => "Apple".to_owned(),
                                        UserManagementAuthenticationProvider::BitbucketOAuth => "Bitbucket".to_owned(),
                                        UserManagementAuthenticationProvider::GitHubOAuth => "GitHub".to_owned(),
                                        UserManagementAuthenticationProvider::GitLabOAuth => "GitLab".to_owned(),
                                        UserManagementAuthenticationProvider::GoogleOAuth => "Google".to_owned(),
                                        UserManagementAuthenticationProvider::IntuitOAuth => "Intuit".to_owned(),
                                        UserManagementAuthenticationProvider::LinkedInOAuth => "LinkedIn".to_owned(),
                                        UserManagementAuthenticationProvider::MicrosoftOAuth => "Microsoft".to_owned(),
                                        UserManagementAuthenticationProvider::SalesforceOAuth => "Salesforce".to_owned(),
                                        UserManagementAuthenticationProvider::SlackOAuth => "Slack".to_owned(),
                                        UserManagementAuthenticationProvider::VercelMarketplaceOAuth => "Vercel Marketplace".to_owned(),
                                        UserManagementAuthenticationProvider::VercelOAuth => "Vercel".to_owned(),
                                        UserManagementAuthenticationProvider::XeroOAuth => "Xero".to_owned(),
                                        UserManagementAuthenticationProvider::Unknown(value) => value.replace("OAuth", ""),
                                        _ => "Unknown".to_owned()
                                    }
                                )
                                .to_owned(),
                            }),
                            addon_start: None,
                            addon_end: None,
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
                    addon_start: None,
                    addon_end: None,
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: format!("Continue with {}", connection.name).to_owned(),
                    }),
                    addon_start: None,
                    addon_end: None,
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
                    .list_users(ListUsersParams {
                        limit: Some(1),
                        email: Some(email),
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
                    .get_authorization_url(GetAuthorizationUrlParams {
                        provider: Some(oauth_provider),
                        // TODO: State and code challenge.
                        ..GetAuthorizationUrlParams::new(&self.options.redirect_url)
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
                    .get_authorization_url(GetAuthorizationUrlParams {
                        connection_id: Some(connection_id),
                        // TODO: State and code challenge.
                        ..GetAuthorizationUrlParams::new(&self.options.redirect_url)
                    })
                    .expect("TODO: handle error");

                Ok(Response::new(ResponseType::Redirect(
                    authorization_url.to_string(),
                )))
            }
        }
    }
}

erased_method_action!(WorkosIndexAction);
