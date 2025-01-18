use bon::Builder;
use openidconnect::{
    core::{
        CoreClient, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJwsSigningAlgorithm,
        CoreProviderMetadata,
    },
    reqwest::async_http_client,
    AuthUrl, ClientId, ClientSecret, IssuerUrl, JsonWebKeySet, RedirectUrl, TokenUrl, UserInfoUrl,
};
use shield::{ConfigurationError, Subprovider};

use crate::provider::OIDC_PROVIDER_ID;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OidcProviderVisibility {
    Public,
    Unlisted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OidcProviderPkceCodeChallenge {
    None,
    Plain,
    S256,
}

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct OidcSubprovider {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub icon_url: Option<String>,
    #[builder(default = OidcProviderVisibility::Public)]
    pub visibility: OidcProviderVisibility,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub redirect_url: Option<String>,
    pub discovery_url: Option<String>,
    pub issuer_url: Option<String>,
    pub authorization_url: Option<String>,
    pub authorization_url_params: Option<String>,
    pub token_url: Option<String>,
    pub token_url_params: Option<String>,
    pub introspection_url: Option<String>,
    pub introspection_url_params: Option<String>,
    pub revocation_url: Option<String>,
    pub revocation_url_params: Option<String>,
    pub user_info_url: Option<String>,
    pub json_web_key_set_url: Option<String>,
    pub json_web_key_set: Option<
        JsonWebKeySet<
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
            CoreJsonWebKeyUse,
            CoreJsonWebKey,
        >,
    >,
    #[builder(default = OidcProviderPkceCodeChallenge::S256)]
    pub pkce_code_challenge: OidcProviderPkceCodeChallenge,
}

impl OidcSubprovider {
    pub async fn oidc_client(&self) -> Result<CoreClient, ConfigurationError> {
        let mut client = if let Some(discovery_url) = &self.discovery_url {
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

        if let Some(redirect_url) = &self.redirect_url {
            client = client.set_redirect_uri(
                RedirectUrl::new(redirect_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
            );
        }

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

    fn icon_url(&self) -> Option<String> {
        self.icon_url.clone()
    }

    fn form(&self) -> Option<shield::Form> {
        None
    }
}
