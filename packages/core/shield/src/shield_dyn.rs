use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    error::ShieldError,
    provider::{Provider, ProviderVisualisation},
    request::{SignInCallbackRequest, SignInRequest},
    response::Response,
    session::Session,
    shield::Shield,
    user::User,
};

#[async_trait]
pub trait DynShield: Send + Sync {
    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError>;

    async fn provider_visualisations(&self) -> Result<Vec<ProviderVisualisation>, ShieldError>;

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError>;

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
    ) -> Result<Response, ShieldError>;

    async fn sign_out(&self, session: Session) -> Result<Response, ShieldError>;
}

#[async_trait]
impl<U: User> DynShield for Shield<U> {
    async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        self.providers().await
    }

    async fn provider_visualisations(&self) -> Result<Vec<ProviderVisualisation>, ShieldError> {
        self.provider_visualisations().await
    }

    async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.sign_in(request, session).await
    }

    async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.sign_in_callback(request, session).await
    }

    async fn sign_out(&self, session: Session) -> Result<Response, ShieldError> {
        self.sign_out(session).await
    }
}

pub struct ShieldDyn(Arc<dyn DynShield>);

impl ShieldDyn {
    pub fn new(shield: Shield<impl User + 'static>) -> Self {
        Self(Arc::new(shield))
    }

    pub async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        self.0.providers().await
    }

    pub async fn provider_visualisations(&self) -> Result<Vec<ProviderVisualisation>, ShieldError> {
        self.0.provider_visualisations().await
    }

    pub async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.0.sign_in(request, session).await
    }

    pub async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.0.sign_in_callback(request, session).await
    }

    pub async fn sign_out(&self, session: Session) -> Result<Response, ShieldError> {
        self.0.sign_out(session).await
    }
}
