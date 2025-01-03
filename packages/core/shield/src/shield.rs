use std::{collections::HashMap, sync::Arc};

use futures::future::try_join_all;
use tracing::debug;

use crate::{
    error::{ProviderError, SessionError, ShieldError},
    provider::{Provider, Subprovider, SubproviderVisualisation},
    request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
    response::Response,
    session::Session,
    storage::Storage,
    user::User,
};

#[derive(Clone)]
pub struct Shield<U: User> {
    storage: Arc<dyn Storage<U>>,
    providers: Arc<HashMap<String, Arc<dyn Provider>>>,
}

impl<U: User> Shield<U> {
    pub fn new<S>(storage: S, providers: Vec<Arc<dyn Provider>>) -> Self
    where
        S: Storage<U> + 'static,
    {
        Self {
            storage: Arc::new(storage),
            providers: Arc::new(
                providers
                    .into_iter()
                    .map(|provider| (provider.id(), provider))
                    .collect(),
            ),
        }
    }

    pub fn storage(&self) -> &dyn Storage<U> {
        &*self.storage
    }

    pub fn provider_by_id(&self, provider_id: &str) -> Option<&dyn Provider> {
        self.providers.get(provider_id).map(|v| &**v)
    }

    pub async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        try_join_all(
            self.providers
                .values()
                .map(|provider| provider.subproviders()),
        )
        .await
        .map(|subproviders| subproviders.into_iter().flatten().collect::<Vec<_>>())
    }

    pub async fn subprovider_visualisations(
        &self,
    ) -> Result<Vec<SubproviderVisualisation>, ShieldError> {
        self.subproviders().await.map(|subproviders| {
            subproviders
                .into_iter()
                .map(|subprovider| {
                    let provider_id = subprovider.provider_id();
                    let subprovider_id = subprovider.id();

                    SubproviderVisualisation {
                        key: match &subprovider_id {
                            Some(subprovider_id) => format!("{provider_id}-{subprovider_id}"),
                            None => provider_id.clone(),
                        },
                        provider_id,
                        subprovider_id,
                        name: subprovider.name(),
                    }
                })
                .collect()
        })
    }

    pub async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        debug!("sign in {:?}", request);

        let provider = match self.providers.get(&request.provider_id) {
            Some(provider) => provider,
            None => return Err(ProviderError::ProviderNotFound(request.provider_id).into()),
        };

        provider.sign_in(request, session).await
    }

    pub async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        debug!("sign in callback {:?}", request);

        let provider = match self.providers.get(&request.provider_id) {
            Some(provider) => provider,
            None => return Err(ProviderError::ProviderNotFound(request.provider_id).into()),
        };

        provider.sign_in_callback(request, session).await
    }

    pub async fn sign_out(&self, session: Session) -> Result<Response, ShieldError> {
        debug!("sign out");

        let authenticated = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication.clone()
        };

        let response = if let Some(authenticated) = authenticated {
            let provider = match self.providers.get(&authenticated.provider_id) {
                Some(provider) => provider,
                None => {
                    return Err(ProviderError::ProviderNotFound(authenticated.provider_id).into())
                }
            };

            provider
                .sign_out(
                    SignOutRequest {
                        provider_id: authenticated.provider_id,
                        subprovider_id: authenticated.subprovider_id,
                    },
                    session.clone(),
                )
                .await?
        } else {
            // TODO: Should be configurable.
            Response::Redirect("/".to_owned())
        };

        session.purge().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        provider::tests::{TestProvider, TEST_PROVIDER_ID},
        storage::tests::{TestStorage, TEST_STORAGE_ID},
    };

    use super::Shield;

    #[test]
    fn test_storage() {
        let shield = Shield::new(TestStorage::default(), vec![]);

        assert_eq!(TEST_STORAGE_ID, shield.storage().id());
    }

    #[test]
    fn test_providers() {
        let shield = Shield::new(
            TestStorage::default(),
            vec![
                Arc::new(TestProvider::default().with_id("test1")),
                Arc::new(TestProvider::default().with_id("test2")),
            ],
        );

        assert_eq!(
            None,
            shield
                .provider_by_id(TEST_PROVIDER_ID)
                .map(|provider| provider.id())
        );
        assert_eq!(
            Some("test1".to_owned()),
            shield.provider_by_id("test1").map(|provider| provider.id())
        );
        assert_eq!(
            Some("test2".to_owned()),
            shield.provider_by_id("test2").map(|provider| provider.id())
        );
    }
}
