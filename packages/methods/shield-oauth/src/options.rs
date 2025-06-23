use bon::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct OauthOptions {
    #[builder(default = "/")]
    pub sign_in_redirect: String,
}

impl Default for OauthOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
