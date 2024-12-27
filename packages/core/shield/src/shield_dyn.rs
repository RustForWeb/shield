use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    error::ShieldError,
    provider::{Subprovider, SubproviderVisualisation},
    request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
    response::Response,
    session::Session,
    shield::Shield,
    user::User,
};

#[async_trait]
pub trait DynShield: Send + Sync {
    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError>;

    async fn subprovider_visualisations(
        &self,
    ) -> Result<Vec<SubproviderVisualisation>, ShieldError>;

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

    async fn sign_out(
        &self,
        request: SignOutRequest,
        session: Session,
    ) -> Result<Response, ShieldError>;
}

#[async_trait]
impl<U: User> DynShield for Shield<U> {
    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        self.subproviders().await
    }

    async fn subprovider_visualisations(
        &self,
    ) -> Result<Vec<SubproviderVisualisation>, ShieldError> {
        self.subprovider_visualisations().await
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

    async fn sign_out(
        &self,
        request: SignOutRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.sign_out(request, session).await
    }
}

pub struct ShieldDyn(Arc<dyn DynShield>);

impl ShieldDyn {
    pub fn new(shield: Shield<impl User + 'static>) -> Self {
        Self(Arc::new(shield))
    }

    pub async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        self.0.subproviders().await
    }

    pub async fn subprovider_visualisations(
        &self,
    ) -> Result<Vec<SubproviderVisualisation>, ShieldError> {
        self.0.subprovider_visualisations().await
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

    pub async fn sign_out(
        &self,
        request: SignOutRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        self.0.sign_out(request, session).await
    }
}
