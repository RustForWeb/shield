use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::error::SessionError;

#[async_trait]
pub trait SessionStorage: Send + Sync {
    fn data(&self) -> Arc<Mutex<SessionData>>;

    async fn update(&self) -> Result<(), SessionError>;

    async fn renew(&self) -> Result<(), SessionError>;

    async fn purge(&self) -> Result<(), SessionError>;
}

#[derive(Clone)]
pub struct Session(Arc<dyn SessionStorage>);

impl Session {
    pub fn new<S: SessionStorage + 'static>(storage: S) -> Self {
        Session(Arc::new(storage))
    }

    pub fn data(&self) -> Arc<Mutex<SessionData>> {
        self.0.data()
    }

    pub async fn update(&self) -> Result<(), SessionError> {
        self.0.update().await
    }

    pub async fn renew(&self) -> Result<(), SessionError> {
        self.0.renew().await
    }

    pub async fn purge(&self) -> Result<(), SessionError> {
        self.0.purge().await
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SessionData {
    pub redirect_url: Option<String>,
    pub authentication: Option<Authentication>,
    pub methods: HashMap<String, String>,
}

impl SessionData {
    pub fn method<T: Default + DeserializeOwned>(
        &self,
        method_id: &str,
    ) -> Result<T, SessionError> {
        match self.methods.get(method_id) {
            Some(value) => serde_json::from_str(value)
                .map_err(|err| SessionError::Serialization(err.to_string())),
            None => Ok(T::default()),
        }
    }

    pub fn set_method<T: Serialize>(
        &mut self,
        method_id: &str,
        value: T,
    ) -> Result<(), SessionError> {
        self.methods.insert(
            method_id.to_owned(),
            serde_json::to_string(&value)
                .map_err(|err| SessionError::Serialization(err.to_string()))?,
        );

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Authentication {
    pub method_id: String,
    pub provider_id: Option<String>,
    pub user_id: String,
}
