use async_trait::async_trait;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreGenderClaim, CoreTokenResponse},
    reqwest::async_http_client,
    url::form_urlencoded::parse,
    AccessToken, AuthorizationCode, CsrfToken, EmptyAdditionalClaims, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse, UserInfoClaims,
};
use shield::{
    Authentication, ConfigurationError, CreateEmailAddress, CreateUser, Provider, ProviderError,
    Response, Session, SessionError, ShieldError, ShieldOptions, SignInCallbackRequest,
    SignInRequest, SignOutRequest, Subprovider, UpdateUser, User,
};
use tracing::debug;

use crate::{
    claims::Claims, storage::OidcStorage, subprovider::OidcSubprovider, CreateOidcConnection,
    OidcConnection, OidcProviderPkceCodeChallenge, UpdateOidcConnection,
};

pub const OIDC_PROVIDER_ID: &str = "oidc";

pub struct OidcProvider<U: User> {
    subproviders: Vec<OidcSubprovider>,
    storage: Box<dyn OidcStorage<U>>,
}

impl<U: User> OidcProvider<U> {
    pub fn new<S: OidcStorage<U> + 'static>(storage: S) -> Self {
        Self {
            subproviders: vec![],
            storage: Box::new(storage),
        }
    }

    pub fn with_subproviders<I: IntoIterator<Item = OidcSubprovider>>(
        mut self,
        subproviders: I,
    ) -> Self {
        self.subproviders = subproviders.into_iter().collect();
        self
    }

    async fn oidc_subprovider_by_id_or_slug(
        &self,
        subprovider_id: &str,
    ) -> Result<OidcSubprovider, ShieldError> {
        if let Some(subprovider) = self
            .subproviders
            .iter()
            .find(|subprovider| subprovider.id == subprovider_id)
        {
            return Ok(subprovider.clone());
        }

        if let Some(subprovider) = self
            .storage
            .oidc_subprovider_by_id_or_slug(subprovider_id)
            .await?
        {
            return Ok(subprovider);
        }

        Err(ProviderError::SubproviderNotFound(subprovider_id.to_owned()).into())
    }

    async fn create_user(&self, claims: &Claims) -> Result<U, ShieldError> {
        if let Some(email) = claims.email() {
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
                            name: claims
                                .name()
                                .and_then(|name| name.get(None).map(|name| name.to_string())),
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

    async fn update_user(&self, user_id: &str, claims: &Claims) -> Result<U, ShieldError> {
        self.storage
            .update_user(UpdateUser {
                id: user_id.to_owned(),
                name: claims
                    .name()
                    .map(|name| name.get(None).map(|name| name.to_string())),
            })
            .await
            .map_err(ShieldError::Storage)
    }

    async fn create_oidc_connection(
        &self,
        subprovider_id: String,
        user_id: String,
        identifier: String,
        token_response: CoreTokenResponse,
    ) -> Result<OidcConnection, ShieldError> {
        let (token_type, access_token, refresh_token, id_token, expired_at, scopes) =
            parse_token_response(token_response)?;

        self.storage
            .create_oidc_connection(CreateOidcConnection {
                identifier,
                token_type,
                access_token,
                refresh_token,
                id_token,
                expired_at,
                scopes,
                subprovider_id,
                user_id,
            })
            .await
            .map_err(ShieldError::Storage)
    }

    async fn update_oidc_connection(
        &self,
        connection_id: String,
        token_response: CoreTokenResponse,
    ) -> Result<OidcConnection, ShieldError> {
        let (token_type, access_token, refresh_token, id_token, expired_at, scopes) =
            parse_token_response(token_response)?;

        self.storage
            .update_oidc_connection(UpdateOidcConnection {
                id: connection_id,
                token_type: Some(token_type),
                access_token: Some(access_token),
                refresh_token: refresh_token.map(Some),
                id_token: id_token.map(Some),
                expired_at: expired_at.map(Some),
                scopes: scopes.map(Some),
            })
            .await
            .map_err(ShieldError::Storage)
    }
}

#[async_trait]
impl<U: User> Provider for OidcProvider<U> {
    fn id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        let subproviders = self
            .subproviders
            .iter()
            .cloned()
            .chain(self.storage.oidc_subproviders().await?);

        Ok(subproviders
            .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            .collect())
    }

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, ShieldError> {
        self.oidc_subprovider_by_id_or_slug(subprovider_id)
            .await
            .map(|subprovider| Some(Box::new(subprovider) as Box<dyn Subprovider>))
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id_or_slug(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        let client = subprovider.oidc_client().await?;

        let mut authorization_request = client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );

        let pkce_code_challenge = match subprovider.pkce_code_challenge {
            OidcProviderPkceCodeChallenge::None => None,
            OidcProviderPkceCodeChallenge::Plain => Some(PkceCodeChallenge::new_random_plain()),
            OidcProviderPkceCodeChallenge::S256 => Some(PkceCodeChallenge::new_random_sha256()),
        };

        if let Some((pkce_code_challenge, _)) = &pkce_code_challenge {
            authorization_request =
                authorization_request.set_pkce_challenge(pkce_code_challenge.clone());
        }

        if let Some(scopes) = subprovider.scopes {
            authorization_request =
                authorization_request.add_scopes(scopes.into_iter().map(Scope::new));
        }

        if let Some(authorization_url_params) = subprovider.authorization_url_params {
            let params = parse(authorization_url_params.trim_start_matches('?').as_bytes());

            for (name, value) in params {
                authorization_request =
                    authorization_request.add_extra_param(name.into_owned(), value.into_owned());
            }
        }

        let (auth_url, csrf_token, nonce) = authorization_request.url();

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = None;

            session_data.csrf = Some(csrf_token.secret().clone());
            session_data.nonce = Some(nonce.secret().clone());
            session_data.verifier = pkce_code_challenge
                .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone());
            session_data.oidc_connection_id = None;
        }

        Ok(Response::Redirect(auth_url.to_string()))
    }

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let (pkce_verifier, csrf, nonce) = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            (
                session_data.verifier.clone(),
                session_data.csrf.clone(),
                session_data.nonce.clone(),
            )
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

        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id_or_slug(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        let client = subprovider.oidc_client().await?;

        let mut token_request =
            client.exchange_code(AuthorizationCode::new(authorization_code.to_owned()));

        if let Some(pkce_verifier) = pkce_verifier {
            token_request = token_request.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier));
        } else if subprovider.pkce_code_challenge != OidcProviderPkceCodeChallenge::None {
            return Err(ShieldError::Validation("Missing PKCE verifier.".to_owned()));
        }

        if let Some(token_url_params) = subprovider.token_url_params {
            let params = parse(token_url_params.trim_start_matches('?').as_bytes());

            for (name, value) in params {
                token_request =
                    token_request.add_extra_param(name.into_owned(), value.into_owned());
            }
        }

        let token_response = token_request
            .request_async(async_http_client)
            .await
            .map_err(|err| ShieldError::Request(err.to_string()))?;

        let claims = if let Some(id_token) = token_response.id_token() {
            let claims = id_token
                .claims(
                    &client.id_token_verifier(),
                    &Nonce::new(
                        nonce
                            .ok_or_else(|| ShieldError::Validation("Missing nonce.".to_owned()))?,
                    ),
                )
                .map_err(|err| ShieldError::Validation(err.to_string()))?;

            Claims::from(claims.clone())
        } else {
            let claims: UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim> = client
                .user_info(token_response.access_token().to_owned(), None)
                .map_err(|err| ConfigurationError::Missing(err.to_string()))?
                .request_async(async_http_client)
                .await
                .map_err(|err| ShieldError::Request(err.to_string()))?;

            Claims::from(claims)
        };

        debug!("{:?}\n{:?}", claims.subject(), claims);

        let (connection, user) = match self
            .storage
            .oidc_connection_by_identifier(&subprovider.id, claims.subject())
            .await?
        {
            Some(connection) => {
                let connection = self
                    .update_oidc_connection(connection.id, token_response)
                    .await?;

                let user = self.update_user(&connection.user_id, &claims).await?;

                (connection, user)
            }
            None => {
                let user = self.create_user(&claims).await?;

                let connection = self
                    .create_oidc_connection(
                        subprovider.id.clone(),
                        user.id(),
                        claims.subject().to_string(),
                        token_response,
                    )
                    .await?;

                (connection, user)
            }
        };

        debug!("signed in {:?} {:?}", user.id(), connection);

        session.renew().await?;

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.csrf = None;
            session_data.nonce = None;
            session_data.verifier = None;

            session_data.authentication = Some(Authentication {
                provider_id: self.id(),
                subprovider_id: Some(subprovider.id),
                user_id: user.id(),
            });
            session_data.oidc_connection_id = Some(connection.id);
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
        session: Session,
        options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id_or_slug(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        let connection_id = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.oidc_connection_id.clone()
        };

        if let Some(connection_id) = connection_id {
            if let Some(connection) = self.storage.oidc_connection_by_id(&connection_id).await? {
                debug!("revoking access token {:?}", connection.access_token);

                let token = AccessToken::new(connection.access_token);

                let client = subprovider.oidc_client().await?;

                let revocation_request = match client.revoke_token(token.into()) {
                    Ok(revocation_request) => Some(revocation_request),
                    Err(openidconnect::ConfigurationError::MissingUrl("revocation")) => None,
                    Err(err) => return Err(ConfigurationError::Invalid(err.to_string()).into()),
                };

                if let Some(revocation_request) = revocation_request {
                    let mut revocation_request = revocation_request;

                    if let Some(revocation_url_params) = subprovider.revocation_url_params {
                        let params =
                            parse(revocation_url_params.trim_start_matches('?').as_bytes());

                        for (name, value) in params {
                            revocation_request = revocation_request
                                .add_extra_param(name.into_owned(), value.into_owned());
                        }
                    }

                    revocation_request
                        .request_async(async_http_client)
                        .await
                        .map_err(|err| ShieldError::Request(err.to_string()))?;
                }
            }
        }

        Ok(Response::Redirect(options.sign_out_redirect.clone()))
    }
}

type ParsedTokenResponse = (
    String,
    String,
    Option<String>,
    Option<String>,
    Option<DateTime<FixedOffset>>,
    Option<Vec<String>>,
);

fn parse_token_response(
    token_response: CoreTokenResponse,
) -> Result<ParsedTokenResponse, ShieldError> {
    Ok((
        token_response.token_type().as_ref().to_string(),
        token_response.access_token().secret().clone(),
        token_response
            .refresh_token()
            .map(|refresh_token| refresh_token.secret().clone()),
        token_response
            .id_token()
            .map(|id_token| id_token.to_string()),
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
