use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shield::{SessionData, SessionError, SessionStorage};
use tower_sessions::Session;

#[derive(Clone, Debug)]
pub struct TowerSessionStorage {
    session: Session,
    session_key: &'static str,
    session_data: Arc<Mutex<SessionData>>,
}

impl TowerSessionStorage {
    pub async fn load(session: Session, session_key: &'static str) -> Result<Self, SessionError> {
        let data = Self::load_data(&session, session_key).await?;

        Ok(Self {
            session,
            session_key,
            session_data: Arc::new(Mutex::new(data)),
        })
    }

    async fn load_data(
        session: &Session,
        session_key: &'static str,
    ) -> Result<SessionData, SessionError> {
        session
            .get::<SessionData>(session_key)
            .await
            .map_err(|err| SessionError::Engine(err.to_string()))
            .map(|session_data| session_data.unwrap_or_default())
    }
}

#[async_trait]
impl SessionStorage for TowerSessionStorage {
    fn data(&self) -> Arc<Mutex<SessionData>> {
        self.session_data.clone()
    }

    async fn update(&self) -> Result<(), SessionError> {
        let data = self
            .session_data
            .lock()
            .map_err(|err| SessionError::Lock(err.to_string()))?
            .clone();

        self.session
            .insert(self.session_key, data)
            .await
            .map_err(|err| SessionError::Engine(err.to_string()))
    }

    async fn renew(&self) -> Result<(), SessionError> {
        self.session
            .cycle_id()
            .await
            .map_err(|err| SessionError::Engine(err.to_string()))
    }

    async fn purge(&self) -> Result<(), SessionError> {
        self.session
            .flush()
            .await
            .map_err(|err| SessionError::Engine(err.to_string()))?;

        let data = Self::load_data(&self.session, self.session_key).await?;

        {
            let mut session_data = self
                .session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;
            *session_data = data;
        }

        Ok(())
    }
}
