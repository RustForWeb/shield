use bon::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct ShieldOptions {}

impl Default for ShieldOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
