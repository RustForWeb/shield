use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{error::SessionError, user::User};

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
    pub base: BaseSession,
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

    pub(crate) fn method_str(&self, method_id: &str) -> Option<&str> {
        self.methods.get(method_id).map(String::as_str)
    }

    pub(crate) fn set_method_str(&mut self, method_id: &str, value: &str) {
        self.methods.insert(method_id.to_owned(), value.to_owned());
    }
}

#[derive(Clone, Debug)]
pub struct MethodSession<'a, S> {
    pub base: &'a BaseSession,
    pub method: &'a S,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BaseSession {
    pub authentication: Option<Authentication>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Authentication {
    pub method_id: String,
    pub provider_id: Option<String>,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub enum SessionAction {
    Authenticate { user_id: String },
    Unauthenticate,
    Data(String),
}

impl SessionAction {
    pub fn authenticate<U: User>(user: U) -> Self {
        Self::Authenticate { user_id: user.id() }
    }

    pub fn unauthenticate() -> Self {
        Self::Unauthenticate
    }

    pub fn data<T: Serialize>(value: T) -> Result<Self, SessionError> {
        let value = serde_json::to_string(&value)
            .map_err(|err| SessionError::Serialization(err.to_string()))?;

        Ok(Self::Data(value))
    }

    pub(crate) async fn call(
        &self,
        method_id: &str,
        provider_id: Option<&str>,
        session: &Session,
    ) -> Result<(), SessionError> {
        match self {
            Self::Authenticate { user_id } => {
                session.renew().await?;

                {
                    let session_data = session.data();
                    let mut session_data = session_data
                        .lock()
                        .map_err(|err| SessionError::Lock(err.to_string()))?;

                    session_data.base.authentication = Some(Authentication {
                        method_id: method_id.to_owned(),
                        provider_id: provider_id.map(ToOwned::to_owned),
                        user_id: user_id.clone(),
                    });
                }

                session.update().await?;
            }
            Self::Unauthenticate => {
                session.purge().await?;
            }
            Self::Data(value) => {
                {
                    let session_data = session.data();
                    let mut session_data = session_data
                        .lock()
                        .map_err(|err| SessionError::Lock(err.to_string()))?;

                    session_data.set_method_str(method_id, value);
                }

                session.update().await?;
            }
        }

        Ok(())
    }
}
