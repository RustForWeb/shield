use openidconnect::{
    core::{CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJwsSigningAlgorithm},
    JsonWebKeySet,
};
use shield::Subprovider;

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
