use bon::Builder;
use workos_sdk::user_management::OauthProvider;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct WorkosOptions {
    #[builder(default)]
    pub(crate) oauth_providers: Vec<OauthProvider>,
}

impl Default for WorkosOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
