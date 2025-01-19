use bon::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct ShieldOptions {
    #[builder(default = "/")]
    pub sign_in_redirect: String,
    #[builder(default = "/")]
    pub sign_out_redirect: String,
}

impl Default for ShieldOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
