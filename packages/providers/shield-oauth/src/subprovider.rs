use shield::{Form, Subprovider};

use crate::provider::OAUTH_PROVIDER_ID;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OauthProviderVisibility {
    Private,
    Public,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OauthProviderPkceCodeChallenge {
    None,
    Plain,
    S256,
}

// TODO: Remove allow dead code.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct OauthSubprovider {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) slug: Option<String>,
    pub(crate) visibility: OauthProviderVisibility,
    pub(crate) client_id: String,
    pub(crate) client_secret: Option<String>,
    pub(crate) scopes: Option<Vec<String>>,
    pub(crate) redirect_url: Option<String>,
    pub(crate) authorization_url: Option<String>,
    pub(crate) authorization_url_params: Option<String>,
    pub(crate) token_url: Option<String>,
    pub(crate) token_url_params: Option<String>,
    pub(crate) introspection_url: Option<String>,
    pub(crate) introspection_url_params: Option<String>,
    pub(crate) revocation_url: Option<String>,
    pub(crate) revocation_url_params: Option<String>,
    pub(crate) pkce_code_challenge: OauthProviderPkceCodeChallenge,
    pub(crate) icon_url: Option<String>,
}

impl Subprovider for OauthSubprovider {
    fn provider_id(&self) -> String {
        OAUTH_PROVIDER_ID.to_owned()
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

    fn form(&self) -> Option<Form> {
        None
    }
}
