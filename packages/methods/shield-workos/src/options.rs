use bon::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct WorkosOptions {}

impl Default for WorkosOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
