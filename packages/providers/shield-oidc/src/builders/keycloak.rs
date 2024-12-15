use crate::subprovider::{OidcProviderPkceCodeChallenge, OidcProviderVisibility, OidcSubprovider};

#[derive(Debug)]
pub struct KeycloakBuilder {
    id: String,
    name: String,
    issuer_url: String,
    client_id: String,
    client_secret: Option<String>,
}

impl KeycloakBuilder {
    pub fn new(id: &str, issuer_url: &str, client_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            name: "Keycloak".to_owned(),
            issuer_url: issuer_url.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(client_secret.to_owned());
        self
    }

    pub fn build(self) -> OidcSubprovider {
        OidcSubprovider {
            id: self.id,
            name: self.name,
            slug: None,
            visibility: OidcProviderVisibility::Public,
            client_id: self.client_id,
            client_secret: self.client_secret,
            scopes: None,
            redirect_url: None,
            issuer_url: Some(self.issuer_url),
            authorization_url: None,
            authorization_url_params: None,
            token_url: None,
            token_url_params: None,
            introspection_url: None,
            introspection_url_params: None,
            revocation_url: None,
            revocation_url_params: None,
            user_info_url: None,
            json_web_key_set_url: None,
            json_web_key_set: None,
            pkce_code_challenge: OidcProviderPkceCodeChallenge::S256,
        }
    }
}
