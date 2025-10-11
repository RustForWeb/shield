use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use shield::{Action, Method, ShieldError, User, erased_method};

use crate::{
    actions::{CredentialsSignInAction, CredentialsSignOutAction},
    credentials::Credentials,
    provider::CredentialsProvider,
};

pub const CREDENTIALS_METHOD_ID: &str = "credentials";

pub struct CredentialsMethod<U: User, D: DeserializeOwned> {
    credentials: Arc<dyn Credentials<U, D>>,
}

impl<U: User, D: DeserializeOwned> CredentialsMethod<U, D> {
    pub fn new<C: Credentials<U, D> + 'static>(credentials: C) -> Self {
        Self {
            credentials: Arc::new(credentials),
        }
    }
}

#[async_trait]
impl<U: User + 'static, D: DeserializeOwned + 'static> Method for CredentialsMethod<U, D> {
    type Provider = CredentialsProvider;
    type Session = ();

    fn id(&self) -> String {
        CREDENTIALS_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn Action<Self::Provider, Self::Session>>> {
        vec![
            Box::new(CredentialsSignInAction::new(self.credentials.clone())),
            Box::new(CredentialsSignOutAction),
        ]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(vec![CredentialsProvider])
    }
}

erased_method!(CredentialsMethod, <U: User, D: DeserializeOwned>);
