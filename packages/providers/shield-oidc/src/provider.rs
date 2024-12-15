use async_trait::async_trait;
use shield::{
    Provider, SignInError, SignInRequest, SignOutError, SignOutRequest, StorageError, Subprovider,
};

use crate::storage::OidcStorage;

pub const OIDC_PROVIDER_ID: &str = "oidc";

pub struct OidcProvider<'a> {
    storage: &'a dyn OidcStorage,
}

impl<'a> OidcProvider<'a> {
    pub fn new<S: OidcStorage + 'static>(storage: &'a S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Provider for OidcProvider<'_> {
    fn id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError> {
        self.storage.oidc_subproviders().await.map(|subproviders| {
            subproviders
                .into_iter()
                .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
                .collect()
        })
    }

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, StorageError> {
        self.storage
            .oidc_subprovider_by_id(subprovider_id)
            .await
            .map(|subprovider| {
                subprovider.map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            })
    }

    async fn sign_in(&self, request: SignInRequest) -> Result<(), SignInError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => match self.subprovider_by_id(&subprovider_id).await? {
                Some(subprovider) => Some(subprovider),
                None => return Err(SignInError::SubproviderNotFound(subprovider_id)),
            },
            None => None,
        };

        todo!()
    }

    async fn sign_out(&self, request: SignOutRequest) -> Result<(), SignOutError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => match self.subprovider_by_id(&subprovider_id).await? {
                Some(subprovider) => Some(subprovider),
                None => return Err(SignOutError::SubproviderNotFound(subprovider_id)),
            },
            None => None,
        };

        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct OidcSubprovider {
    id: String,
    name: String,
}

impl Subprovider for OidcSubprovider {
    fn provider_id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn form(&self) -> Option<shield::Form> {
        None
    }
}
