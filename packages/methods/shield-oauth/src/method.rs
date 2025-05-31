use async_trait::async_trait;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
    basic::BasicTokenResponse, url::form_urlencoded::parse,
};
use shield::{
    Authentication, ConfigurationError, CreateEmailAddress, CreateUser, Method, Provider,
    ProviderError, Response, Session, SessionError, ShieldError, ShieldOptions,
    SignInCallbackRequest, SignInRequest, SignOutRequest, UpdateUser, User,
};

use crate::{
    CreateOauthConnection, OauthConnection, UpdateOauthConnection,
    client::async_http_client,
    provider::{OauthProvider, OauthProviderPkceCodeChallenge},
    session::OauthSession,
    storage::OauthStorage,
};

pub const OAUTH_METHOD_ID: &str = "oauth";

pub struct OauthMethod<U: User> {
    providers: Vec<OauthProvider>,
    storage: Box<dyn OauthStorage<U>>,
}

impl<U: User> OauthMethod<U> {
    pub fn new<S: OauthStorage<U> + 'static>(storage: S) -> Self {
        Self {
            providers: vec![],
            storage: Box::new(storage),
        }
    }

    pub fn with_providers<I: IntoIterator<Item = OauthProvider>>(mut self, providers: I) -> Self {
        self.providers = providers.into_iter().collect();
        self
    }

    async fn oauth_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<OauthProvider, ShieldError> {
        if let Some(provider) = self
            .providers
            .iter()
            .find(|provider| provider.id == provider_id)
        {
            return Ok(provider.clone());
        }

        if let Some(provider) = self
            .storage
            .oauth_provider_by_id_or_slug(provider_id)
            .await?
        {
            return Ok(provider);
        }

        Err(ProviderError::ProviderNotFound(provider_id.to_owned()).into())
    }

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
impl<U: User> Method for OauthMethod<U> {
    fn id(&self) -> String {
        OAUTH_METHOD_ID.to_owned()
    }

    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        let providers = self
            .providers
            .iter()
            .cloned()
            .chain(self.storage.oauth_providers().await?);

        Ok(providers
            .map(|provider| Box::new(provider) as Box<dyn Provider>)
            .collect())
    }

    async fn provider_by_id(
        &self,
        provider_id: &str,
    ) -> Result<Option<Box<dyn Provider>>, ShieldError> {
        self.oauth_provider_by_id_or_slug(provider_id)
            .await
            .map(|provider| Some(Box::new(provider) as Box<dyn Provider>))
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id_or_slug(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

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

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = None;

            session_data.set_method(
                OAUTH_METHOD_ID,
                OauthSession {
                    csrf: Some(csrf_token.secret().clone()),
                    pkce_verifier: pkce_code_challenge
                        .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone()),
                    oauth_connection_id: None,
                },
            )?;
        }

        Ok(Response::Redirect(auth_url.to_string()))
    }

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
        options: &ShieldOptions,
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
            .as_ref()
            .and_then(|query| query.get("state"))
            .and_then(|code| code.as_str())
            .ok_or_else(|| ShieldError::Validation("Missing state.".to_owned()))?;

        if csrf.is_none_or(|csrf| csrf != state) {
            return Err(ShieldError::Validation("Invalid state.".to_owned()));
        }

        let authorization_code = request
            .query
            .as_ref()
            .and_then(|query| query.get("code"))
            .and_then(|code| code.as_str())
            .ok_or_else(|| ShieldError::Validation("Missing authorization code.".to_owned()))?;

        let provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id_or_slug(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

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

        Ok(Response::Redirect(
            request
                .redirect_url
                .unwrap_or(options.sign_in_redirect.clone()),
        ))
    }

    async fn sign_out(
        &self,
        request: SignOutRequest,
        _session: Session,
        _options: &ShieldOptions,
    ) -> Result<Option<Response>, ShieldError> {
        let _provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id_or_slug(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

        // TODO: OAuth token revocation.

        Ok(None)
    }
}

type ParsedTokenResponse = (
    String,
    String,
    Option<String>,
    Option<DateTime<FixedOffset>>,
    Option<Vec<String>>,
);

fn parse_token_response(
    token_response: BasicTokenResponse,
) -> Result<ParsedTokenResponse, ShieldError> {
    Ok((
        token_response.token_type().as_ref().to_string(),
        token_response.access_token().secret().clone(),
        token_response
            .refresh_token()
            .map(|refresh_token| refresh_token.secret().clone()),
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
