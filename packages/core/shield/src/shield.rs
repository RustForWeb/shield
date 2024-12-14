use std::collections::HashMap;

use crate::{provider::Provider, storage::Storage};

pub struct Shield {
    storage: Box<dyn Storage>,
    providers: HashMap<&'static str, Box<dyn Provider>>,
}

impl Shield {
    pub fn new<S>(storage: S, providers: Vec<Box<dyn Provider>>) -> Self
    where
        S: Storage + 'static,
    {
        Self {
            storage: Box::new(storage),
            providers: providers
                .into_iter()
                .map(|provider| (provider.id(), provider))
                .collect(),
        }
    }

    pub fn storage(&self) -> &dyn Storage {
        &*self.storage
    }

    pub fn provider_by_id(&self, provider_id: &str) -> Option<&dyn Provider> {
        self.providers.get(provider_id).map(|v| &**v)
    }

    pub fn sign_in(&self, _provider_id: &str) {
        todo!()
    }

    pub fn sign_out(&self, _provider_id: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
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
                Box::new(TestProvider::default().with_id("test1")),
                Box::new(TestProvider::default().with_id("test2")),
            ],
        );

        assert_eq!(
            None,
            shield
                .provider_by_id(TEST_PROVIDER_ID)
                .map(|provider| provider.id())
        );
        assert_eq!(
            Some("test1"),
            shield.provider_by_id("test1").map(|provider| provider.id())
        );
        assert_eq!(
            Some("test2"),
            shield.provider_by_id("test2").map(|provider| provider.id())
        );
    }
}
