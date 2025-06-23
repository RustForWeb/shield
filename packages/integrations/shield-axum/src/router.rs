use axum::{
    Router,
    routing::{get, post},
};
use shield::User;

use crate::routes::*;

#[cfg_attr(feature = "utoipa", derive(utoipa::OpenApi))]
#[cfg_attr(feature = "utoipa", openapi(paths()))]
pub struct AuthRoutes;

impl AuthRoutes {
    pub fn router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>() -> Router<S> {
        Router::new()
            .route("/{methodId}/{actionId}", get(action::<U>))
            .route("/{methodId}/{actionId}", post(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", get(action::<U>))
            .route("/{methodId}/{actionId}/{providerId}", post(action::<U>))
    }
}
