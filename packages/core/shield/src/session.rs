use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::SessionError;

#[async_trait]
pub trait SessionStorage: Send + Sync {
    async fn load(&self) -> Result<SessionData, SessionError>;

    async fn store(&self, session_data: SessionData) -> Result<(), SessionError>;

    async fn renew(&self) -> Result<(), SessionError>;

    async fn purge(&self) -> Result<(), SessionError>;
}

#[derive(Clone)]
pub struct Session(Arc<dyn SessionStorage>);

impl Session {
    pub fn new<S: SessionStorage + 'static>(storage: S) -> Self {
        Session(Arc::new(storage))
    }

    pub async fn load(&self) -> Result<SessionData, SessionError> {
        self.0.load().await
    }

    pub async fn update(&self, session_data: SessionData) -> Result<(), SessionError> {
        self.0.store(session_data).await
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
