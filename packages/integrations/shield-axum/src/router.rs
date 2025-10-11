use axum::{
    Router,
    routing::{get, post},
};
use shield::User;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::routes::*;

#[cfg_attr(feature = "utoipa", derive(utoipa::OpenApi))]
#[cfg_attr(feature = "utoipa", openapi(paths(action, forms, user)))]
pub struct AuthRoutes;

impl AuthRoutes {
    pub fn router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>() -> Router<S> {
        Router::new()
            .route("/user", get(user::<U>))
            .route("/forms/{actionId}", get(forms::<U>))
            .route("/{methodId}/{actionId}", get(action::<U>))
            .route("/{methodId}/{actionId}", post(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", get(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", post(action::<U>))
    }

    #[cfg(feature = "utoipa")]
    pub fn openapi_router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>()
    -> OpenApiRouter<S> {
        OpenApiRouter::with_openapi(AuthRoutes::openapi())
            .route("/user", get(user::<U>))
            .route("/forms/{actionId}", get(forms::<U>))
            .route("/{methodId}/{actionId}", get(action::<U>))
            .route("/{methodId}/{actionId}", post(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", get(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", post(action::<U>))
    }
}
