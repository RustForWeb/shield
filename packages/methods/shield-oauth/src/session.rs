use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OauthSession {
    pub redirect_origin: Option<Url>,
    pub csrf: Option<String>,
    pub pkce_verifier: Option<String>,
    pub oauth_connection_id: Option<String>,
}
