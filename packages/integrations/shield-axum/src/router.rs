use axum::{
    Router,
    routing::{any, get, post},
};
use shield::{Shield, User};
#[cfg(feature = "utoipa")]
use utoipa::OpenApi;
#[cfg(feature = "utoipa")]
use utoipa_axum::router::OpenApiRouter;

use crate::routes::*;

#[cfg(feature = "utoipa")]
#[cfg_attr(feature = "utoipa", derive(utoipa::OpenApi))]
#[cfg_attr(feature = "utoipa", openapi(paths(action, forms, user)))]
struct BaseOpenApi;

pub struct AuthRoutes<U: User> {
    shield: Shield<U>,
}

impl<U: Clone + User + 'static> AuthRoutes<U> {
    pub fn new(shield: Shield<U>) -> Self {
        Self { shield }
    }

    pub fn router<S: Clone + Send + Sync + 'static>(&self) -> Router<S> {
        Router::new()
            .route("/user", get(user::<U>))
            .route("/forms/{actionId}", get(forms::<U>))
            .route("/{methodId}/{actionId}", any(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", any(action::<U>))
    }

    #[cfg(feature = "utoipa")]
    pub fn openapi_router<S: Clone + Send + Sync + 'static>(&self) -> OpenApiRouter<S> {
        OpenApiRouter::with_openapi(BaseOpenApi::openapi().merge_from(self.shield.openapi()))
            .route("/user", get(user::<U>))
            .route("/forms/{actionId}", get(forms::<U>))
            .route("/{methodId}/{actionId}", get(action::<U>))
            .route("/{methodId}/{actionId}", post(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", get(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", post(action::<U>))
    }
}
