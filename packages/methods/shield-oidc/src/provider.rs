use bon::Builder;
use openidconnect::{
    AuthUrl, Client, ClientId, ClientSecret, EmptyAdditionalClaims, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IntrospectionUrl, IssuerUrl, JsonWebKeySet, JsonWebKeySetUrl,
    RedirectUrl, RevocationUrl, StandardErrorResponse, TokenUrl, UserInfoUrl,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreErrorResponseType, CoreGenderClaim,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
        CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse,
        CoreTokenResponse,
    },
};
use secrecy::{ExposeSecret, SecretString};
use shield::{ConfigurationError, Provider};

use crate::{
    client::async_http_client,
    metadata::{NonStandardProviderMetadata, OidcProviderMetadata},
    method::OIDC_METHOD_ID,
};

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
#[expect(clippy::duplicated_attributes)]
#[derive(Builder, Clone, Debug)]
#[builder(
    on(String, into),
    on(SecretString, into),
    state_mod(vis = "pub(crate)")
)]
pub struct OidcProvider {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub icon_url: Option<String>,
    #[builder(default = OidcProviderVisibility::Public)]
    pub visibility: OidcProviderVisibility,
    pub client_id: String,
    pub client_secret: Option<SecretString>,
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

impl OidcProvider {
    pub async fn oidc_client(&self) -> Result<OidcClient, ConfigurationError> {
        let async_http_client = async_http_client()?;

        let provider_metadata = if let Some(discovery_url) = &self.discovery_url {
            OidcProviderMetadata::discover_async(
                // TODO: Consider stripping `/.well-known/openid-configuration` so `openidconnect` doesn't error.
                IssuerUrl::new(discovery_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
                &async_http_client,
            )
            .await
            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?
        } else {
            let mut provider_metadata = OidcProviderMetadata::new(
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
                NonStandardProviderMetadata {
                    introspection_endpoint: self
                        .introspection_url
                        .as_ref()
                        .map(|introspection_url| {
                            IntrospectionUrl::new(introspection_url.clone())
                                .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                        })
                        .transpose()?,
                    revocation_endpoint: self
                        .revocation_url
                        .as_ref()
                        .map(|revocation_url| {
                            RevocationUrl::new(revocation_url.clone())
                                .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                        })
                        .transpose()?,
                },
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
            self.client_secret
                .clone()
                .map(|client_secret| ClientSecret::new(client_secret.expose_secret().to_owned())),
        );

        // TODO: Upstream: _option version of these (and other) functions which set the type to EndpointMaybeSet.

        // if let Some(introspection_endpoint) = provider_metadata
        //     .additional_metadata()
        //     .introspection_endpoint
        // {
        //     client = client.set_introspection_url(introspection_endpoint);
        // }
        // if let Some(revocation_url) = provider_metadata.additional_metadata().revocation_endpoint {
        //     client = client.set_introspection_url(revocation_url);
        // }

        if let Some(redirect_url) = &self.redirect_url {
            client = client.set_redirect_uri(
                RedirectUrl::new(redirect_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
            );
        }

        // TODO: Client options.

        Ok(client)
    }
}

impl Provider for OidcProvider {
    fn method_id(&self) -> String {
        OIDC_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
