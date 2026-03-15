use std::sync::Arc;

use async_trait::async_trait;
use shield::{Method, MethodAction, ShieldError, Storage, User, erased_method};

use crate::{actions::DummySignInAction, provider::DummyProvider};

pub const DUMMY_METHOD_ID: &str = "dummy";

pub struct DummyMethod<U: User> {
    storage: Arc<dyn Storage<U>>,
}

impl<U: User> DummyMethod<U> {
    pub fn new<S: Storage<U> + 'static>(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
        }
    }
}

#[async_trait]
impl<U: User + 'static> Method for DummyMethod<U> {
    type Provider = DummyProvider;
    type Session = ();

    fn id(&self) -> String {
        DUMMY_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn MethodAction<Self::Provider, Self::Session>>> {
        vec![Box::new(DummySignInAction::new(self.storage.clone()))]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(vec![DummyProvider])
    }
}

erased_method!(DummyMethod, <U: User>);
