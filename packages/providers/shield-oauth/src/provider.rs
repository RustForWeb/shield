use async_trait::async_trait;
use shield::{
    Provider, ProviderError, Response, Session, ShieldError, SignInCallbackRequest, SignInRequest,
    SignOutRequest, Subprovider, User,
};

use crate::{storage::OauthStorage, subprovider::OauthSubprovider};

pub const OAUTH_PROVIDER_ID: &str = "oauth";

pub struct OauthProvider<U: User> {
    subproviders: Vec<OauthSubprovider>,
    storage: Box<dyn OauthStorage<U>>,
}

impl<U: User> OauthProvider<U> {
    pub fn new<S: OauthStorage<U> + 'static>(storage: S) -> Self {
        Self {
            subproviders: vec![],
            storage: Box::new(storage),
        }
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

        if let Some(subprovider) = self.storage.oauth_subprovider_by_id(subprovider_id).await? {
            return Ok(subprovider);
        }

        Err(ProviderError::SubproviderNotFound(subprovider_id.to_owned()).into())
    }
}

#[async_trait]
impl<U: User> Provider for OauthProvider<U> {
    fn id(&self) -> String {
        OAUTH_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        let subproviders = self
            .subproviders
            .iter()
            .cloned()
            .chain(self.storage.oauth_subproviders().await?);

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

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => self.oauth_subprovider_by_id(&subprovider_id).await?,
            None => return Err(ProviderError::SubproviderMissing.into()),
        };

        todo!("oauth sign in callback")
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
