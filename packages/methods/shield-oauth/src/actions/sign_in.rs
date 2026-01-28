use async_trait::async_trait;
use oauth2::{CsrfToken, PkceCodeChallenge, Scope, url::form_urlencoded::parse};
use serde::Deserialize;
use shield::{
    Action, ActionMethod, ConfigurationError, Form, Input, InputType, InputTypeHidden,
    InputTypeSubmit, InputValue, MethodSession, Provider, Request, Response, ResponseType,
    SessionAction, ShieldError, SignInAction, erased_action,
};
use url::Url;

use crate::{
    options::OauthOptions,
    provider::{OauthProvider, OauthProviderPkceCodeChallenge},
    session::OauthSession,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInData {
    pub redirect_origin: Url,
    pub redirect_url: Option<String>,
}

pub struct OauthSignInAction {
    options: OauthOptions,
}

impl OauthSignInAction {
    pub fn new(options: OauthOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Action<OauthProvider, OauthSession> for OauthSignInAction {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in with OAuth"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in with OAuth."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    async fn forms(&self, provider: OauthProvider) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![Form {
            inputs: vec![
                Input {
                    name: "redirectOrigin".to_owned(),
                    label: None,
                    r#type: InputType::Hidden(InputTypeHidden::default()),
                    value: Some(InputValue::Origin),
                },
                Input {
                    name: "redirectUrl".to_owned(),
                    label: None,
                    r#type: InputType::Hidden(InputTypeHidden::default()),
                    value: Some(InputValue::Query {
                        key: "redirectUrl".to_owned(),
                    }),
                },
                Input {
                    name: "submit".to_owned(),
                    label: None,
                    r#type: InputType::Submit(InputTypeSubmit::default()),
                    value: Some(InputValue::String {
                        value: format!("Sign in with {}", provider.name()),
                    }),
                },
            ],
        }])
    }

    async fn call(
        &self,
        provider: OauthProvider,
        _session: &MethodSession<OauthSession>,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let data = serde_json::from_value::<SignInData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let redirect_url = data
            .redirect_url
            .map(|redirect_url| data.redirect_origin.join(&redirect_url))
            .unwrap_or_else(|| data.redirect_origin.join(&self.options.sign_in_redirect))
            .map_err(|err| ShieldError::Validation(format!("redirect URL parse error: {err}")))?;

        if let Some(redirect_origins) = &self.options.redirect_origins {
            let redirect_origin = Url::parse(&redirect_url.origin().ascii_serialization())
                .map_err(|err| {
                    ShieldError::Validation(format!("redirect origin parse error: {err}"))
                })?;

            if !redirect_origins.contains(&redirect_origin) {
                return Err(ShieldError::Validation(format!(
                    "redirect origin `{redirect_origin}` not allowed"
                )));
            }
        }

        let client = provider.oauth_client().await?;

        let mut authorization_request = client
            .authorize_url(CsrfToken::new_random)
            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?;

        let pkce_code_challenge = match provider.pkce_code_challenge {
            OauthProviderPkceCodeChallenge::None => None,
            OauthProviderPkceCodeChallenge::Plain => Some(PkceCodeChallenge::new_random_plain()),
            OauthProviderPkceCodeChallenge::S256 => Some(PkceCodeChallenge::new_random_sha256()),
        };

        if let Some((pkce_code_challenge, _)) = &pkce_code_challenge {
            authorization_request =
                authorization_request.set_pkce_challenge(pkce_code_challenge.clone());
        }

        if let Some(scopes) = provider.scopes {
            authorization_request =
                authorization_request.add_scopes(scopes.into_iter().map(Scope::new));
        }

        if let Some(authorization_url_params) = provider.authorization_url_params {
            let params = parse(authorization_url_params.trim_start_matches('?').as_bytes());

            for (name, value) in params {
                authorization_request =
                    authorization_request.add_extra_param(name.into_owned(), value.into_owned());
            }
        }

        let (auth_url, csrf_token) = authorization_request.url();

        Ok(Response::new(ResponseType::Redirect(auth_url.to_string()))
            .session_action(SessionAction::Unauthenticate)
            .session_action(SessionAction::data(OauthSession {
                redirect_url: Some(redirect_url),
                csrf: Some(csrf_token.secret().clone()),
                pkce_verifier: pkce_code_challenge
                    .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone()),
                oauth_connection_id: None,
            })?))
    }
}

erased_action!(OauthSignInAction);
