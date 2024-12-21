use async_trait::async_trait;
use shield::{
    Provider, ProviderError, Response, Session, ShieldError, SignInRequest, SignOutRequest,
    Subprovider,
};

use crate::{storage::OauthStorage, subprovider::OauthSubprovider};

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

    async fn oauth_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<OauthSubprovider, ShieldError> {
        if let Some(subprovider) = self
            .subproviders
            .iter()
            .find(|subprovider| subprovider.id == subprovider_id)
        {
            return Ok(subprovider.clone());
        }

        if let Some(storage) = &self.storage {
            if let Some(subprovider) = storage.oauth_subprovider_by_id(subprovider_id).await? {
                return Ok(subprovider);
            }
        }

        Err(ProviderError::SubproviderNotFound(subprovider_id.to_owned()).into())
    }
}

#[async_trait]
impl Provider for OauthProvider {
    fn id(&self) -> String {
        OAUTH_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
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
    ) -> Result<Option<Box<dyn Subprovider>>, ShieldError> {
        self.oauth_subprovider_by_id(subprovider_id)
            .await
            .map(|subprovider| Some(Box::new(subprovider) as Box<dyn Subprovider>))
    }
    async fn sign_in(
        &self,
        request: SignInRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oauth_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        todo!("oauth sign in")
    }

    async fn sign_out(
        &self,
        request: SignOutRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oauth_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        todo!("oauth sign out")
    }
}
