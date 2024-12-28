use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use shield::{Session, Shield, User};

pub struct ExtractShield<U: User>(pub Shield<U>);

#[async_trait]
impl<S: Send + Sync, U: User + Clone + 'static> FromRequestParts<S> for ExtractShield<U> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Shield<U>>()
            .cloned()
            .map(ExtractShield)
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract Shield. Is `ShieldLayer` enabled?",
            ))
    }
}

pub struct ExtractSession(pub Session);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ExtractSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Session>()
            .cloned()
            .map(ExtractSession)
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract Shield session. Is `ShieldLayer` enabled?",
            ))
    }
}

pub struct ExtractUser<U: User>(pub Option<U>);

#[async_trait]
impl<S: Send + Sync, U: User + Clone + 'static> FromRequestParts<S> for ExtractUser<U> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Option<U>>()
            .cloned()
            .map(ExtractUser)
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract Shield user. Is `ShieldLayer` enabled?",
            ))
    }
}
