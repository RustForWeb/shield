use std::{collections::HashMap, sync::Arc};

use futures::future::try_join_all;

use crate::{
    error::{ProviderError, ShieldError},
    provider::{Provider, Subprovider, SubproviderVisualisation},
    request::{SignInRequest, SignOutRequest},
    storage::Storage,
};

#[derive(Clone)]
pub struct Shield {
    storage: Arc<dyn Storage>,
    providers: Arc<HashMap<String, Arc<dyn Provider>>>,
}

impl Shield {
    pub fn new<S>(storage: S, providers: Vec<Arc<dyn Provider>>) -> Self
    where
        S: Storage + 'static,
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

    pub fn storage(&self) -> &dyn Storage {
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

    pub async fn sign_in(&self, request: SignInRequest) -> Result<(), ShieldError> {
        let provider = match self.providers.get(&request.provider_id) {
            Some(provider) => provider,
            None => return Err(ProviderError::ProviderNotFound(request.provider_id).into()),
        };

        // let subprovider = match request.subprovider_id {
        //     Some(subprovider_id) => match provider.subprovider_by_id(&subprovider_id).await? {
        //         Some(subprovider) => Some(subprovider),
        //         None => return Err(SignInError::SubproviderNotFound(subprovider_id)),
        //     },
        //     None => None,
        // };

        println!("sign in {:?}", request);

        provider.sign_in(request).await
    }

    pub async fn sign_out(&self, _request: SignOutRequest) -> Result<(), ShieldError> {
        todo!()
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
