use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Method, ShieldError, User, erased_method};

use crate::{
    actions::{OidcSignInAction, OidcSignInCallbackAction, OidcSignOutAction},
    options::OidcOptions,
    provider::OidcProvider,
    session::OidcSession,
    storage::OidcStorage,
};

pub const OIDC_METHOD_ID: &str = "oidc";

pub struct OidcMethod<U: User> {
    options: OidcOptions,
    providers: Vec<OidcProvider>,
    storage: Arc<dyn OidcStorage<U>>,
}

impl<U: User> OidcMethod<U> {
    pub fn new<S: OidcStorage<U> + 'static>(storage: S) -> Self {
        Self {
            options: OidcOptions::default(),
            providers: vec![],
            storage: Arc::new(storage),
        }
    }

    pub fn with_options(mut self, options: OidcOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_providers<I: IntoIterator<Item = OidcProvider>>(mut self, providers: I) -> Self {
        self.providers = providers.into_iter().collect();
        self
    }

    async fn oidc_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<Option<OidcProvider>, ShieldError> {
        if let Some(provider) = self
            .providers
            .iter()
            .find(|provider| provider.id == provider_id)
        {
            return Ok(Some(provider.clone()));
        }

        if let Some(provider) = self
            .storage
            .oidc_provider_by_id_or_slug(provider_id)
            .await?
        {
            return Ok(Some(provider));
        }

        Ok(None)
    }
}

#[async_trait]
impl<U: User + 'static> Method for OidcMethod<U> {
    type Provider = OidcProvider;
    type Session = OidcSession;

    fn id(&self) -> String {
        OIDC_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn Action<Self::Provider, Self::Session>>> {
        vec![
            Box::new(OidcSignInAction::new(self.options.clone())),
            Box::new(OidcSignInCallbackAction::new(
                self.options.clone(),
                self.storage.clone(),
            )),
            Box::new(OidcSignOutAction),
        ]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(self
            .providers
            .iter()
            .cloned()
            .chain(self.storage.oidc_providers().await?)
            .collect())
    }

    async fn provider_by_id(
        &self,
        provider_id: Option<&str>,
    ) -> Result<Option<Self::Provider>, ShieldError> {
        if let Some(provider_id) = provider_id {
            self.oidc_provider_by_id_or_slug(provider_id).await
        } else {
            Ok(None)
        }
    }
}

erased_method!(OidcMethod, <U: User>);
