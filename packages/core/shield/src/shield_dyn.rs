use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{
    action::ActionForms, error::ShieldError, session::Session, shield::Shield, user::User,
};

#[async_trait]
pub trait DynShield: Send + Sync {
    async fn providers(&self) -> Result<Vec<Box<dyn Any + Send + Sync>>, ShieldError>;

    async fn action_forms(
        &self,
        action_id: &str,
        session: Session,
    ) -> Result<ActionForms, ShieldError>;
}

#[async_trait]
impl<U: User> DynShield for Shield<U> {
    async fn providers(&self) -> Result<Vec<Box<dyn Any + Send + Sync>>, ShieldError> {
        self.providers().await
    }

    async fn action_forms(
        &self,
        action_id: &str,
        session: Session,
    ) -> Result<ActionForms, ShieldError> {
        self.action_forms(action_id, session).await
    }
}

pub struct ShieldDyn(Arc<dyn DynShield>);

impl ShieldDyn {
    pub fn new(shield: Shield<impl User + 'static>) -> Self {
        Self(Arc::new(shield))
    }

    pub async fn providers(&self) -> Result<Vec<Box<dyn Any + Send + Sync>>, ShieldError> {
        self.0.providers().await
    }

    pub async fn action_forms(
        &self,
        action_id: &str,
        session: Session,
    ) -> Result<ActionForms, ShieldError> {
        self.0.action_forms(action_id, session).await
    }
}
