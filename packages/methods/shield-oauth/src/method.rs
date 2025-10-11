use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Method, ShieldError, User, erased_method};

use crate::{
    actions::{OauthSignInAction, OauthSignInCallbackAction, OauthSignOutAction},
    options::OauthOptions,
    provider::OauthProvider,
    session::OauthSession,
    storage::OauthStorage,
};

pub const OAUTH_METHOD_ID: &str = "oauth";

pub struct OauthMethod<U: User> {
    options: OauthOptions,
    providers: Vec<OauthProvider>,
    storage: Arc<dyn OauthStorage<U>>,
}

impl<U: User> OauthMethod<U> {
    pub fn new<S: OauthStorage<U> + 'static>(storage: S) -> Self {
        Self {
            options: OauthOptions::default(),
            providers: vec![],
            storage: Arc::new(storage),
        }
    }

    pub fn with_options(mut self, options: OauthOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_providers<I: IntoIterator<Item = OauthProvider>>(mut self, providers: I) -> Self {
        self.providers = providers.into_iter().collect();
        self
    }

    async fn oauth_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<Option<OauthProvider>, ShieldError> {
        if let Some(provider) = self
            .providers
            .iter()
            .find(|provider| provider.id == provider_id)
        {
            return Ok(Some(provider.clone()));
        }

        if let Some(provider) = self
            .storage
            .oauth_provider_by_id_or_slug(provider_id)
            .await?
        {
            return Ok(Some(provider));
        }

        Ok(None)
    }
}

#[async_trait]
impl<U: User + 'static> Method for OauthMethod<U> {
    type Provider = OauthProvider;
    type Session = OauthSession;

    fn id(&self) -> String {
        OAUTH_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn Action<Self::Provider, Self::Session>>> {
        vec![
            Box::new(OauthSignInAction),
            Box::new(OauthSignInCallbackAction::new(
                self.options.clone(),
                self.storage.clone(),
            )),
            Box::new(OauthSignOutAction),
        ]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(self
            .providers
            .iter()
            .cloned()
            .chain(self.storage.oauth_providers().await?)
            .collect())
    }

    async fn provider_by_id(
        &self,
        provider_id: Option<&str>,
    ) -> Result<Option<Self::Provider>, ShieldError> {
        if let Some(provider_id) = provider_id {
            self.oauth_provider_by_id_or_slug(provider_id).await
        } else {
            Ok(None)
        }
    }
}

erased_method!(OauthMethod, <U: User>);
