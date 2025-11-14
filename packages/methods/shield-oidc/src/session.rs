use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OidcSession {
    pub redirect_origin: Option<Url>,
    pub csrf: Option<String>,
    pub nonce: Option<String>,
    pub pkce_verifier: Option<String>,
    pub oidc_connection_id: Option<String>,
}
