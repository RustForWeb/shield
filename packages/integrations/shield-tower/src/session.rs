use async_trait::async_trait;
use shield::{SessionData, SessionError, SessionStorage};
use tower_sessions::Session;

#[derive(Clone, Debug)]
pub struct TowerSession {
    session: Session,
    session_key: &'static str,
}

impl TowerSession {
    pub fn new(session: Session, session_key: &'static str) -> Self {
        Self {
            session,
            session_key,
        }
    }
}

#[async_trait]
impl SessionStorage for TowerSession {
    async fn load(&self) -> Result<SessionData, SessionError> {
        self.session
            .get::<SessionData>(self.session_key)
            .await
            .map_err(|err| SessionError::Engine(err.to_string()))
            .map(|session_data| session_data.unwrap_or_default())
    }

    async fn store(&self, session_data: SessionData) -> Result<(), SessionError> {
        self.session
            .insert(self.session_key, &session_data)
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
            .map_err(|err| SessionError::Engine(err.to_string()))
    }
}
