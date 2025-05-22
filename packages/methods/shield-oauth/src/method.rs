use async_trait::async_trait;
use shield::{
    Method, Provider, ProviderError, Response, Session, ShieldError, ShieldOptions,
    SignInCallbackRequest, SignInRequest, SignOutRequest, User,
};

use crate::{provider::OauthProvider, storage::OauthStorage};

pub const OAUTH_METHOD_ID: &str = "oauth";

pub struct OauthMethod<U: User> {
    providers: Vec<OauthProvider>,
    storage: Box<dyn OauthStorage<U>>,
}

impl<U: User> OauthMethod<U> {
    pub fn new<S: OauthStorage<U> + 'static>(storage: S) -> Self {
        Self {
            providers: vec![],
            storage: Box::new(storage),
        }
    }

    pub fn with_providers<I: IntoIterator<Item = OauthProvider>>(mut self, providers: I) -> Self {
        self.providers = providers.into_iter().collect();
        self
    }

    async fn oauth_provider_by_id(&self, provider_id: &str) -> Result<OauthProvider, ShieldError> {
        if let Some(provider) = self
            .providers
            .iter()
            .find(|provider| provider.id == provider_id)
        {
            return Ok(provider.clone());
        }

        if let Some(provider) = self.storage.oauth_provider_by_id(provider_id).await? {
            return Ok(provider);
        }

        Err(ProviderError::ProviderNotFound(provider_id.to_owned()).into())
    }
}

#[async_trait]
impl<U: User> Method for OauthMethod<U> {
    fn id(&self) -> String {
        OAUTH_METHOD_ID.to_owned()
    }

    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        let providers = self
            .providers
            .iter()
            .cloned()
            .chain(self.storage.oauth_providers().await?);

        Ok(providers
            .map(|provider| Box::new(provider) as Box<dyn Provider>)
            .collect())
    }

    async fn provider_by_id(
        &self,
        provider_id: &str,
    ) -> Result<Option<Box<dyn Provider>>, ShieldError> {
        self.oauth_provider_by_id(provider_id)
            .await
            .map(|provider| Some(Box::new(provider) as Box<dyn Provider>))
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        _session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let _provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

        todo!("oauth sign in")
    }

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        _session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let _provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

        todo!("oauth sign in callback")
    }

    async fn sign_out(
        &self,
        request: SignOutRequest,
        _session: Session,
        _options: &ShieldOptions,
    ) -> Result<Response, ShieldError> {
        let _provider = match request.provider_id {
            Some(provider_id) => self.oauth_provider_by_id(&provider_id).await?,
            None => return Err(ProviderError::ProviderMissing.into()),
        };

        todo!("oauth sign out")
    }
}
