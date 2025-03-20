use axum::{extract::FromRequestParts, http::request::Parts};
use shield::{ConfigurationError, Session, Shield, ShieldError, User};

use crate::error::RouteError;

pub struct ExtractShield<U: User>(pub Shield<U>);

impl<S: Send + Sync, U: User + Clone + 'static> FromRequestParts<S> for ExtractShield<U> {
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Shield<U>>()
            .cloned()
            .map(ExtractShield)
            .ok_or(ShieldError::Configuration(ConfigurationError::Invalid(
                "Can't extract Shield. Is `ShieldLayer` enabled?".to_owned(),
            )))
            .map_err(RouteError::from)
    }
}

pub struct ExtractSession(pub Session);

impl<S: Send + Sync> FromRequestParts<S> for ExtractSession {
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Session>()
            .cloned()
            .map(ExtractSession)
            .ok_or(ShieldError::Configuration(ConfigurationError::Invalid(
                "Can't extract Shield. Is `ShieldLayer` enabled?".to_owned(),
            )))
            .map_err(RouteError::from)
    }
}

pub struct ExtractUser<U: User>(pub Option<U>);

impl<S: Send + Sync, U: User + Clone + 'static> FromRequestParts<S> for ExtractUser<U> {
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Option<U>>()
            .cloned()
            .map(ExtractUser)
            .ok_or(ShieldError::Configuration(ConfigurationError::Invalid(
                "Can't extract Shield. Is `ShieldLayer` enabled?".to_owned(),
            )))
            .map_err(RouteError::from)
    }
}

pub struct UserRequired<U: User>(pub U);

impl<S: Send + Sync, U: User + Clone + 'static> FromRequestParts<S> for UserRequired<U> {
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Option<U>>()
            .cloned()
            .ok_or(ShieldError::Configuration(ConfigurationError::Invalid(
                "Can't extract Shield. Is `ShieldLayer` enabled?".to_owned(),
            )))
            .and_then(|user| user.ok_or(ShieldError::Unauthorized))
            .map(UserRequired)
            .map_err(RouteError::from)
    }
}
