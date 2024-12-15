use openidconnect::{
    core::{
        CoreClient, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJwsSigningAlgorithm,
        CoreProviderMetadata,
    },
    reqwest::async_http_client,
    AuthUrl, ClientId, ClientSecret, IssuerUrl, JsonWebKeySet, TokenUrl, UserInfoUrl,
};
use shield::{ConfigurationError, Subprovider};

use crate::provider::OIDC_PROVIDER_ID;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OidcProviderVisibility {
    Private,
    Public,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OidcProviderPkceCodeChallenge {
    None,
    Plain,
    S256,
}

// TODO: Remove allow dead code.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct OidcSubprovider {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) slug: Option<String>,
    pub(crate) visibility: OidcProviderVisibility,
    pub(crate) client_id: String,
    pub(crate) client_secret: Option<String>,
    pub(crate) scopes: Option<Vec<String>>,
    pub(crate) redirect_url: Option<String>,
    pub(crate) discovery_url: Option<String>,
    pub(crate) issuer_url: Option<String>,
    pub(crate) authorization_url: Option<String>,
    pub(crate) authorization_url_params: Option<String>,
    pub(crate) token_url: Option<String>,
    pub(crate) token_url_params: Option<String>,
    pub(crate) introspection_url: Option<String>,
    pub(crate) introspection_url_params: Option<String>,
    pub(crate) revocation_url: Option<String>,
    pub(crate) revocation_url_params: Option<String>,
    pub(crate) user_info_url: Option<String>,
    pub(crate) json_web_key_set_url: Option<String>,
    pub(crate) json_web_key_set: Option<
        JsonWebKeySet<
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
            CoreJsonWebKeyUse,
            CoreJsonWebKey,
        >,
    >,
    pub(crate) pkce_code_challenge: OidcProviderPkceCodeChallenge,
}

impl OidcSubprovider {
    pub async fn oidc_client(&self) -> Result<CoreClient, ConfigurationError> {
        let client = if let Some(discovery_url) = &self.discovery_url {
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
                ClientId::new(self.client_id.clone()),
                self.client_secret.clone().map(ClientSecret::new),
            )
        } else {
            CoreClient::new(
                ClientId::new(self.client_id.clone()),
                self.client_secret.clone().map(ClientSecret::new),
                IssuerUrl::new(
                    self.issuer_url
                        .clone()
                        .ok_or(ConfigurationError::Missing("issuer URL".to_owned()))?,
                )
                .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                self.authorization_url
                    .as_ref()
                    .ok_or(ConfigurationError::Missing("authorization URL".to_owned()))
                    .and_then(|authorization_url| {
                        AuthUrl::new(authorization_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                    })?,
                match &self.token_url {
                    Some(token_url) => Some(
                        TokenUrl::new(token_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                    ),
                    None => None,
                },
                match &self.user_info_url {
                    Some(user_info_url) => Some(
                        UserInfoUrl::new(user_info_url.clone())
                            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                    ),
                    None => None,
                },
                self.json_web_key_set
                    .clone()
                    .ok_or(ConfigurationError::Missing("JSON Web Key Set".to_owned()))?,
            )
        };

        // TODO: Common client options.

        Ok(client)
    }
}

impl Subprovider for OidcSubprovider {
    fn provider_id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn form(&self) -> Option<shield::Form> {
        None
    }
}
