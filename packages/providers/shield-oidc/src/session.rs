use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OidcSession {
    pub csrf: Option<String>,
    pub nonce: Option<String>,
    pub pkce_verifier: Option<String>,
    pub oidc_connection_id: Option<String>,
}
