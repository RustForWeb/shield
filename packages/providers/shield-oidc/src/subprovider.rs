use bon::Builder;
use openidconnect::{
    AuthUrl, Client, ClientId, ClientSecret, EmptyAdditionalClaims,
    EmptyAdditionalProviderMetadata, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl,
    JsonWebKeySet, JsonWebKeySetUrl, ProviderMetadata, RedirectUrl, StandardErrorResponse,
    TokenUrl, UserInfoUrl,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreErrorResponseType, CoreGenderClaim,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
        CoreProviderMetadata, CoreRevocableToken, CoreRevocationErrorResponse,
        CoreTokenIntrospectionResponse, CoreTokenResponse,
    },
};
use shield::{ConfigurationError, Subprovider};

use crate::{client::async_http_client, provider::OIDC_PROVIDER_ID};

type OidcClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

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
    pub json_web_key_set: Option<JsonWebKeySet<CoreJsonWebKey>>,
    #[builder(default = OidcProviderPkceCodeChallenge::S256)]
    pub pkce_code_challenge: OidcProviderPkceCodeChallenge,
}

impl OidcSubprovider {
    pub async fn oidc_client(&self) -> Result<OidcClient, ConfigurationError> {
        let async_http_client = async_http_client()?;

        let provider_metadata = if let Some(discovery_url) = &self.discovery_url {
            CoreProviderMetadata::discover_async(
                // TODO: Consider stripping `/.well-known/openid-configuration` so `openidconnect` doesn't error.
                IssuerUrl::new(discovery_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                &async_http_client,
            )
            .await
            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?
        } else {
            let mut provider_metadata = ProviderMetadata::new(
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
                // Dummy URL, never requested.
                JsonWebKeySetUrl::new("http://127.0.0.1/never-requested".to_owned())
                    .expect("Valid URL."),
                vec![],
                vec![],
                // By default, signing algorithm is not checked, so allowing all possible values should behave the same.
                vec![
                    CoreJwsSigningAlgorithm::HmacSha256,
                    CoreJwsSigningAlgorithm::HmacSha384,
                    CoreJwsSigningAlgorithm::HmacSha512,
                    CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
                    CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha384,
                    CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha512,
                    CoreJwsSigningAlgorithm::EcdsaP256Sha256,
                    CoreJwsSigningAlgorithm::EcdsaP384Sha384,
                    CoreJwsSigningAlgorithm::EcdsaP521Sha512,
                    CoreJwsSigningAlgorithm::RsaSsaPssSha256,
                    CoreJwsSigningAlgorithm::RsaSsaPssSha384,
                    CoreJwsSigningAlgorithm::RsaSsaPssSha512,
                    CoreJwsSigningAlgorithm::EdDsa,
                    CoreJwsSigningAlgorithm::None,
                ],
                EmptyAdditionalProviderMetadata {},
            );

            provider_metadata = provider_metadata.set_jwks(
                self.json_web_key_set
                    .clone()
                    .ok_or(ConfigurationError::Missing("JSON Web Key Set".to_owned()))?,
            );

            if let Some(token_url) = &self.token_url {
                provider_metadata = provider_metadata.set_token_endpoint(Some(
                    TokenUrl::new(token_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                ));
            }

            if let Some(user_info_url) = &self.user_info_url {
                provider_metadata = provider_metadata.set_userinfo_endpoint(Some(
                    UserInfoUrl::new(user_info_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                ));
            }

            provider_metadata
        };

        let mut client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(self.client_id.clone()),
            self.client_secret.clone().map(ClientSecret::new),
        );

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
