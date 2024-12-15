use async_trait::async_trait;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessToken, AuthUrl, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, Scope, TokenUrl,
    UserInfoUrl,
};
use shield::{
    ConfigurationError, Provider, ProviderError, ShieldError, SignInRequest, SignOutRequest,
    Subprovider,
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

    async fn oidc_client_for_subprovider(
        subprovider: &OidcSubprovider,
    ) -> Result<CoreClient, ConfigurationError> {
        let client = if let Some(discovery_url) = &subprovider.discovery_url {
            let provider_metadata = CoreProviderMetadata::discover_async(
                // TODO: Consider stripping `/.well-known/openid-configuration` so `openidconnect` doesn't error.
                IssuerUrl::new(discovery_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                async_http_client,
            )
            .await
            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?;

            CoreClient::from_provider_metadata(
                provider_metadata,
                ClientId::new(subprovider.client_id.clone()),
                subprovider.client_secret.clone().map(ClientSecret::new),
            )
        } else {
            CoreClient::new(
                ClientId::new(subprovider.client_id.clone()),
                subprovider.client_secret.clone().map(ClientSecret::new),
                IssuerUrl::new(
                    subprovider
                        .issuer_url
                        .clone()
                        .ok_or(ConfigurationError::Missing("issuer URL".to_owned()))?,
                )
                .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                subprovider
                    .authorization_url
                    .as_ref()
                    .ok_or(ConfigurationError::Missing("authorization URL".to_owned()))
                    .and_then(|authorization_url| {
                        AuthUrl::new(authorization_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                    })?,
                match &subprovider.token_url {
                    Some(token_url) => Some(
                        TokenUrl::new(token_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                    ),
                    None => None,
                },
                match &subprovider.user_info_url {
                    Some(user_info_url) => Some(
                        UserInfoUrl::new(user_info_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                    ),
                    None => None,
                },
                subprovider
                    .json_web_key_set
                    .clone()
                    .ok_or(ConfigurationError::Missing("JSON Web Key Set".to_owned()))?,
            )
        };

        // TODO: Common client options.

        Ok(client)
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

    async fn sign_in(&self, request: SignInRequest) -> Result<(), ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        let client = Self::oidc_client_for_subprovider(&subprovider).await?;

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

        let (auth_url, csrf_token, nonce) = authorization_request.url();

        // TODO: Store CSRF and nonce in session.
        // TODO: Redirect.

        todo!("redirect {} {:?} {:?}", auth_url, csrf_token, nonce)
    }

    async fn sign_out(&self, request: SignOutRequest) -> Result<(), ShieldError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oidc_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        // TODO: find access token
        let token = AccessToken::new("".to_owned());

        let client = Self::oidc_client_for_subprovider(&subprovider).await?;

        let revocation_request = match client.revoke_token(token.into()) {
            Ok(revocation_request) => revocation_request,
            Err(openidconnect::ConfigurationError::MissingUrl("revocation")) => return Ok(()),
            Err(err) => return Err(ConfigurationError::Invalid(err.to_string()).into()),
        };

        revocation_request
            .request_async(async_http_client)
            .await
            .expect("TODO: revocation request error");

        Ok(())
    }
}
