use std::any::Any;

use shield::{Provider, Shield, SignInRequest, SignOutRequest, Subprovider};

use crate::OauthStorage;

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

    fn subproviders(&self, shield: &Shield) -> Vec<Subprovider> {
        let storage = shield.storage_as::<&dyn OauthStorage>();

        println!("storage {:?}", storage.is_some());

        // let storage = storage
        //     .downcast_ref::<Box<dyn OauthStorage>>()
        //     .expect("Shield storage should implement `OauthStorage` to use `OauthProvider`.");

        vec![]
    }

    fn sign_in(&self, _shield: &Shield, _request: SignInRequest) {
        todo!()
    }

    fn sign_out(&self, _shield: &Shield, _request: SignOutRequest) {
        todo!()
    }
}
