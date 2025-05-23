use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OauthSession {
    pub csrf: Option<String>,
    pub pkce_verifier: Option<String>,
    pub oauth_connection_id: Option<String>,
}
