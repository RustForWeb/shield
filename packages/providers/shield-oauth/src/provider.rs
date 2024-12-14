use async_trait::async_trait;
use shield::{Provider, SignInRequest, SignOutRequest, StorageError, Subprovider, Value};

use crate::storage::OauthStorage;

pub const OAUTH_PROVIDER_ID: &str = "oauth";

pub struct OauthProvider<'a> {
    storage: &'a dyn OauthStorage,
}

impl<'a> OauthProvider<'a> {
    pub fn new<S: OauthStorage + 'static>(storage: &'a S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Provider for OauthProvider<'_> {
    fn id(&self) -> &'static str {
        OAUTH_PROVIDER_ID
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError> {
        self.storage.oauth_subproviders().await.map(|subproviders| {
            subproviders
                .into_iter()
                .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
                .collect()
        })
    }

    async fn sign_in(&self, _request: SignInRequest) {
        todo!()
    }

    async fn sign_out(&self, _request: SignOutRequest) {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct OauthSubprovider {
    id: String,
}

impl Subprovider for OauthSubprovider {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn data(&self) -> Option<Value> {
        None
    }

    fn form(&self) -> Option<shield::Form> {
        None
    }
}
