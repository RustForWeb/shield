use async_trait::async_trait;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreGenderClaim},
    reqwest::async_http_client,
    AccessToken, AuthorizationCode, CsrfToken, EmptyAdditionalClaims, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse, UserInfoClaims,
};
use shield::{
    ConfigurationError, Provider, ProviderError, Response, Session, SessionError, ShieldError,
    SignInCallbackRequest, SignInRequest, SignOutRequest, Subprovider,
};

use crate::{
    claims::Claims, storage::OidcStorage, subprovider::OidcSubprovider,
    OidcProviderPkceCodeChallenge,
};

pub const OIDC_PROVIDER_ID: &str = "oidc";

#[derive(Default)]
pub struct OidcProvider {
    subproviders: Vec<OidcSubprovider>,
    storage: Option<Box<dyn OidcStorage>>,
}

impl OidcProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_storage<S: OidcStorage + 'static>(mut self, storage: S) -> Self {
        self.storage = Some(Box::new(storage));
        self
    }

    pub fn with_subproviders<I: IntoIterator<Item = OidcSubprovider>>(
        mut self,
        subproviders: I,
    ) -> Self {
        self.subproviders = subproviders.into_iter().collect();
        self
    }

    async fn oidc_subprovider_by_id(
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

        if let Some(storage) = &self.storage {
            if let Some(subprovider) = storage.oidc_subprovider_by_id(subprovider_id).await? {
                return Ok(subprovider);
            }
        }

        Err(ProviderError::SubproviderNotFound(subprovider_id.to_owned()).into())
    }
}

#[async_trait]
impl Provider for OidcProvider {
    fn id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        let subproviders =
            self.subproviders
                .iter()
                .cloned()
                .chain(if let Some(storage) = &self.storage {
                    storage.oidc_subproviders().await?
                } else {
                    vec![]
                });

        Ok(subproviders
            .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            .collect())
    }

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, ShieldError> {
        self.oidc_subprovider_by_id(subprovider_id)
            .await
            .map(|subprovider| Some(Box::new(subprovider) as Box<dyn Subprovider>))
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id(&subprovider_id).await?,
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

        let (auth_url, csrf_token, nonce) = authorization_request.url();

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.csrf = Some(csrf_token.secret().clone());
            session_data.nonce = Some(nonce.secret().clone());
            session_data.verifier = pkce_code_challenge
                .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone());
        }

        session.update().await?;

        Ok(Response::Redirect(auth_url.to_string()))
    }

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
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
            .ok_or_else(|| ShieldError::Verification("Missing state.".to_owned()))?;

        if csrf.is_none_or(|csrf| csrf != state) {
            return Err(ShieldError::Verification("Invalid state.".to_owned()));
        }

        let authorization_code = request
            .query
            .as_ref()
            .and_then(|query| query.get("code"))
            .and_then(|code| code.as_str())
            .ok_or_else(|| ShieldError::Verification("Missing authorization code.".to_owned()))?;

        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        let client = subprovider.oidc_client().await?;

        let mut token_request =
            client.exchange_code(AuthorizationCode::new(authorization_code.to_owned()));

        if let Some(pkce_verifier) = pkce_verifier {
            token_request = token_request.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier));
        } else if subprovider.pkce_code_challenge != OidcProviderPkceCodeChallenge::None {
            return Err(ShieldError::Verification(
                "Missing PKCE verifier.".to_owned(),
            ));
        }

        let token_response = token_request
            .request_async(async_http_client)
            .await
            .map_err(|err| ShieldError::Request(err.to_string()))?;

        let claims = if let Some(id_token) = token_response.id_token() {
            let claims =
                id_token
                    .claims(
                        &client.id_token_verifier(),
                        &Nonce::new(nonce.ok_or_else(|| {
                            ShieldError::Verification("Missing nonce.".to_owned())
                        })?),
                    )
                    .map_err(|err| ShieldError::Verification(err.to_string()))?;

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

        println!("{:?}\n{:?}", claims.subject(), claims);

        // TODO
        Ok(Response::Redirect("/".to_owned()))
    }

    async fn sign_out(
        &self,
        request: SignOutRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        // TODO: find access token
        let token = AccessToken::new("".to_owned());

        let client = subprovider.oidc_client().await?;

        let revocation_request = match client.revoke_token(token.into()) {
            Ok(revocation_request) => Some(revocation_request),
            Err(openidconnect::ConfigurationError::MissingUrl("revocation")) => None,
            Err(err) => return Err(ConfigurationError::Invalid(err.to_string()).into()),
        };

        if let Some(revocation_request) = revocation_request {
            revocation_request
                .request_async(async_http_client)
                .await
                .expect("TODO: revocation request error");
        }

        // TODO: This doesn't make sense and/or should be configurable.
        Ok(Response::Redirect("/".to_owned()))
    }
}
