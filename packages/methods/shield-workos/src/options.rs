use bon::Builder;
use workos::UserManagementAuthenticationProvider;

#[derive(Builder, Clone, Debug)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct WorkosOptions {
    #[builder(default)]
    pub(crate) oauth_providers: Vec<UserManagementAuthenticationProvider>,
    // TODO: Generate automatically?
    pub(crate) redirect_url: String,
}
