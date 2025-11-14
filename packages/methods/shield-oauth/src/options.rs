use bon::Builder;
use url::Url;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct OauthOptions {
    #[builder(default = "/")]
    pub(crate) sign_in_redirect: String,

    #[builder(with = FromIterator::from_iter)]
    pub(crate) redirect_origins: Option<Vec<Url>>,
}

impl Default for OauthOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
