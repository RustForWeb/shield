use oauth2::{
    AuthUrl, Client, ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, IntrospectionUrl,
    RedirectUrl, RevocationUrl, StandardRevocableToken, TokenUrl,
    basic::{
        BasicClient, BasicErrorResponse, BasicRevocationErrorResponse,
        BasicTokenIntrospectionResponse, BasicTokenResponse,
    },
};
use secrecy::{ExposeSecret, SecretString};
use shield::{ConfigurationError, Provider};

use crate::method::OAUTH_METHOD_ID;

type OauthClient = Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointMaybeSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OauthProviderVisibility {
    Public,
    Unlisted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OauthProviderPkceCodeChallenge {
    None,
    Plain,
    S256,
}

#[derive(Clone, Debug)]
pub struct OauthProvider {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub visibility: OauthProviderVisibility,
    pub client_id: String,
    pub client_secret: Option<SecretString>,
    pub scopes: Option<Vec<String>>,
    pub redirect_url: Option<String>,
    pub authorization_url: Option<String>,
    pub authorization_url_params: Option<String>,
    pub token_url: Option<String>,
    pub token_url_params: Option<String>,
    pub introspection_url: Option<String>,
    pub introspection_url_params: Option<String>,
    pub revocation_url: Option<String>,
    pub revocation_url_params: Option<String>,
    pub pkce_code_challenge: OauthProviderPkceCodeChallenge,
    pub icon_url: Option<String>,
}

impl OauthProvider {
    pub async fn oauth_client(&self) -> Result<OauthClient, ConfigurationError> {
        let mut client = BasicClient::new(ClientId::new(self.client_id.clone()));

        if let Some(client_secret) = &self.client_secret {
            client = client
                .set_client_secret(ClientSecret::new(client_secret.expose_secret().to_owned()));
        }

        if let Some(redirect_url) = &self.redirect_url {
            client = client.set_redirect_uri(
                RedirectUrl::new(redirect_url.clone())
                    .map_err(|err| ConfigurationError::Invalid(err.to_string()))?,
            );
        }

        let client = client.set_auth_uri_option(
            self.authorization_url
                .as_ref()
                .map(|authorization_url| {
                    AuthUrl::new(authorization_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                })
                .transpose()?,
        );

        let client = client.set_token_uri_option(
            self.token_url
                .as_ref()
                .map(|token_url| {
                    TokenUrl::new(token_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                })
                .transpose()?,
        );

        let client = client.set_introspection_url_option(
            self.introspection_url
                .as_ref()
                .map(|introspection_url| {
                    IntrospectionUrl::new(introspection_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                })
                .transpose()?,
        );

        let client = client.set_revocation_url_option(
            self.revocation_url
                .as_ref()
                .map(|revocation_url| {
                    RevocationUrl::new(revocation_url.clone())
                        .map_err(|err| ConfigurationError::Invalid(err.to_string()))
                })
                .transpose()?,
        );

        Ok(client)
    }
}

impl Provider for OauthProvider {
    fn method_id(&self) -> String {
        OAUTH_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
