use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use oauth2::{
    AuthorizationCode, PkceCodeVerifier, TokenResponse, basic::BasicTokenResponse,
    url::form_urlencoded::parse,
};
use secrecy::SecretString;
use shield::{
    Action, Authentication, ConfigurationError, CreateEmailAddress, CreateUser, Form, Request,
    Response, SIGN_IN_CALLBACK_ACTION_ID, Session, SessionError, ShieldError, UpdateUser, User,
    erased_action,
};

use crate::{
    client::async_http_client,
    connection::{CreateOauthConnection, OauthConnection, UpdateOauthConnection},
    method::OAUTH_METHOD_ID,
    options::OauthOptions,
    provider::{OauthProvider, OauthProviderPkceCodeChallenge},
    session::OauthSession,
    storage::OauthStorage,
};

pub struct OauthSignInCallbackAction<U: User> {
    options: OauthOptions,
    storage: Arc<dyn OauthStorage<U>>,
}

impl<U: User> OauthSignInCallbackAction<U> {
    pub fn new(options: OauthOptions, storage: Arc<dyn OauthStorage<U>>) -> Self {
        Self { options, storage }
    }

    // TODO: Consider if there is a better location for the functions below.

    async fn create_user(&self, email: Option<&str>, name: Option<&str>) -> Result<U, ShieldError> {
        if let Some(email) = email {
            match self.storage.user_by_email(email).await? {
                Some(_) => Err(ShieldError::Validation(
                    "\
                Email address `{email}` is already used by another account. \
                To link a new provider, sign in to with your exising account first. \
                If this is not your account, please contact support for assistence.\
                "
                    .to_owned(),
                )),
                None => Ok(self
                    .storage
                    .create_user(
                        CreateUser {
                            name: name.map(ToOwned::to_owned),
                        },
                        CreateEmailAddress {
                            email: email.to_string(),
                            is_primary: true,
                            // TODO: from claim?
                            is_verified: false,
                            // TODO: generate if not verified
                            verification_token: None,
                            verification_token_expired_at: None,
                            verified_at: None,
                        },
                    )
                    .await?),
            }
        } else {
            Err(ShieldError::Validation(
                "Missing email address in OpenID Connect claims.".to_owned(),
            ))
        }
    }

    async fn update_user(&self, user_id: &str, name: Option<&str>) -> Result<U, ShieldError> {
        self.storage
            .update_user(UpdateUser {
                id: user_id.to_owned(),
                name: name.map(ToOwned::to_owned).map(Some),
            })
            .await
            .map_err(ShieldError::Storage)
    }

    async fn create_oauth_connection(
        &self,
        provider_id: String,
        user_id: String,
        identifier: String,
        token_response: BasicTokenResponse,
    ) -> Result<OauthConnection, ShieldError> {
        let (token_type, access_token, refresh_token, expired_at, scopes) =
            parse_token_response(token_response)?;

        self.storage
            .create_oauth_connection(CreateOauthConnection {
                identifier,
                token_type,
                access_token,
                refresh_token,
                expired_at,
                scopes,
                provider_id,
                user_id,
            })
            .await
            .map_err(ShieldError::Storage)
    }

    async fn update_oauth_connection(
        &self,
        connection_id: String,
        token_response: BasicTokenResponse,
    ) -> Result<OauthConnection, ShieldError> {
        let (token_type, access_token, refresh_token, expired_at, scopes) =
            parse_token_response(token_response)?;

        self.storage
            .update_oauth_connection(UpdateOauthConnection {
                id: connection_id,
                token_type: Some(token_type),
                access_token: Some(access_token),
                refresh_token: refresh_token.map(Some),
                expired_at: expired_at.map(Some),
                scopes: scopes.map(Some),
            })
            .await
            .map_err(ShieldError::Storage)
    }
}

#[async_trait]
impl<U: User + 'static> Action<OauthProvider> for OauthSignInCallbackAction<U> {
    fn id(&self) -> String {
        SIGN_IN_CALLBACK_ACTION_ID.to_owned()
    }

    fn render(&self, _provider: OauthProvider) -> Form {
        Form {
            inputs: vec![],
            attributes: None,
        }
    }

    async fn call(
        &self,
        provider: OauthProvider,
        session: Session,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let OauthSession {
            csrf,
            pkce_verifier,
            ..
        } = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.method(OAUTH_METHOD_ID)?
        };

        let state = request
            .query
            .get("state")
            .and_then(|code| code.as_str())
            .ok_or_else(|| ShieldError::Validation("Missing state.".to_owned()))?;

        if csrf.is_none_or(|csrf| csrf != state) {
            return Err(ShieldError::Validation("Invalid state.".to_owned()));
        }

        let authorization_code = request
            .query
            .get("code")
            .and_then(|code| code.as_str())
            .ok_or_else(|| ShieldError::Validation("Missing authorization code.".to_owned()))?;

        let client = provider.oauth_client().await?;

        let mut token_request = client
            .exchange_code(AuthorizationCode::new(authorization_code.to_owned()))
            .map_err(|err| {
                ShieldError::Configuration(ConfigurationError::Missing(err.to_string()))
            })?;

        if let Some(pkce_verifier) = pkce_verifier {
            token_request = token_request.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier));
        } else if provider.pkce_code_challenge != OauthProviderPkceCodeChallenge::None {
            return Err(ShieldError::Validation("Missing PKCE verifier.".to_owned()));
        }

        if let Some(token_url_params) = provider.token_url_params {
            let params = parse(token_url_params.trim_start_matches('?').as_bytes());

            for (name, value) in params {
                token_request =
                    token_request.add_extra_param(name.into_owned(), value.into_owned());
            }
        }

        let async_http_client = async_http_client()?;

        let token_response = token_request
            .request_async(&async_http_client)
            .await
            .map_err(|err| ShieldError::Request(err.to_string()))?;

        // TODO: user info
        let identifier = "";
        let email = Some("");
        let name = Some("");

        let (connection, user) = match self
            .storage
            .oauth_connection_by_identifier(&provider.id, identifier)
            .await?
        {
            Some(connection) => {
                let connection = self
                    .update_oauth_connection(connection.id, token_response)
                    .await?;

                let user = self.update_user(&connection.user_id, name).await?;

                (connection, user)
            }
            None => {
                let user = self.create_user(email, name).await?;

                let connection = self
                    .create_oauth_connection(
                        provider.id.clone(),
                        user.id(),
                        identifier.to_owned(),
                        token_response,
                    )
                    .await?;

                (connection, user)
            }
        };

        session.renew().await?;

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = Some(Authentication {
                method_id: self.id(),
                provider_id: Some(provider.id),
                user_id: user.id(),
            });

            session_data.set_method(
                OAUTH_METHOD_ID,
                OauthSession {
                    csrf: None,
                    pkce_verifier: None,
                    oauth_connection_id: Some(connection.id),
                },
            )?;
        }

        Ok(Response::Redirect(self.options.sign_in_redirect.clone()))
    }
}

erased_action!(OauthSignInCallbackAction, <U: User>);

type ParsedTokenResponse = (
    String,
    SecretString,
    Option<SecretString>,
    Option<DateTime<FixedOffset>>,
    Option<Vec<String>>,
);

fn parse_token_response(
    token_response: BasicTokenResponse,
) -> Result<ParsedTokenResponse, ShieldError> {
    Ok((
        token_response.token_type().as_ref().to_string(),
        token_response.access_token().secret().as_str().into(),
        token_response
            .refresh_token()
            .map(|refresh_token| refresh_token.secret().as_str().into()),
        match token_response.expires_in() {
            Some(expires_in) => Some(
                (Utc::now()
                    + Duration::from_std(expires_in)
                        .map_err(|err| ShieldError::Validation(err.to_string()))?)
                .into(),
            ),
            None => None,
        },
        token_response
            .scopes()
            .map(|scopes| scopes.iter().map(|scope| scope.to_string()).collect()),
    ))
}
