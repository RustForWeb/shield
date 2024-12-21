use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    pub user_id: Option<String>,
}
