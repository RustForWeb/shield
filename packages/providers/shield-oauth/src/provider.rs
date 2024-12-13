use shield::Provider;

pub const OAUTH_PROVIDER_ID: &str = "oauth";

pub struct OauthProvider {}

impl OauthProvider {
    // pub fn new() -> Self {
    //     Self {}
    // }
}

impl Provider for OauthProvider {
    fn id(&self) -> &'static str {
        OAUTH_PROVIDER_ID
    }

    fn sign_in(&self) {
        todo!()
    }

    fn sign_out(&self) {
        todo!()
    }
}
