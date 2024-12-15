use async_trait::async_trait;
use openidconnect::{
    core::CoreAuthenticationFlow, reqwest::async_http_client, AccessToken, CsrfToken, Nonce, Scope,
};
use shield::{
    ConfigurationError, Provider, ProviderError, Response, ShieldError, SignInRequest,
    SignOutRequest, Subprovider,
};

use crate::{storage::OidcStorage, subprovider::OidcSubprovider};

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

    async fn sign_in(&self, request: SignInRequest) -> Result<Response, ShieldError> {
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

        // TODO: PKCE code challenge.

        if let Some(scopes) = subprovider.scopes {
            authorization_request =
                authorization_request.add_scopes(scopes.into_iter().map(Scope::new));
        }

        let (auth_url, _csrf_token, _nonce) = authorization_request.url();

        // TODO: Store CSRF and nonce in session.
        // TODO: Redirect.

        Ok(Response::Redirect(auth_url.to_string()))
    }

    async fn sign_out(&self, request: SignOutRequest) -> Result<Response, ShieldError> {
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
