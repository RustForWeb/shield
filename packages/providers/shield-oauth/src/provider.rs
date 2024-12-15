use async_trait::async_trait;
use shield::{
    Provider, SignInError, SignInRequest, SignOutError, SignOutRequest, StorageError, Subprovider,
};

use crate::storage::OauthStorage;

pub const OAUTH_PROVIDER_ID: &str = "oauth";

#[derive(Default)]
pub struct OauthProvider {
    subproviders: Vec<OauthSubprovider>,
    storage: Option<Box<dyn OauthStorage>>,
}

impl OauthProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_storage<S: OauthStorage + 'static>(mut self, storage: S) -> Self {
        self.storage = Some(Box::new(storage));
        self
    }

    pub fn with_subproviders<I: IntoIterator<Item = OauthSubprovider>>(
        mut self,
        subproviders: I,
    ) -> Self {
        self.subproviders = subproviders.into_iter().collect();
        self
    }
}

#[async_trait]
impl Provider for OauthProvider {
    fn id(&self) -> String {
        OAUTH_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError> {
        let subproviders =
            self.subproviders
                .iter()
                .cloned()
                .chain(if let Some(storage) = &self.storage {
                    storage.oauth_subproviders().await?
                } else {
                    vec![]
                });

        Ok(subproviders
            .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            .collect())
    }

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, StorageError> {
        if let Some(subprovider) = self
            .subproviders
            .iter()
            .find(|subprovider| subprovider.id == subprovider_id)
        {
            return Ok(Some(Box::new(subprovider.clone()) as Box<dyn Subprovider>));
        }

        if let Some(storage) = &self.storage {
            if let Some(subprovider) = storage.oauth_subprovider_by_id(subprovider_id).await? {
                return Ok(Some(Box::new(subprovider.clone()) as Box<dyn Subprovider>));
            }
        }

        Ok(None)
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
pub struct OauthSubprovider {
    id: String,
    name: String,
}

impl Subprovider for OauthSubprovider {
    fn provider_id(&self) -> String {
        OAUTH_PROVIDER_ID.to_owned()
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
