use axum::{
    Router,
    routing::{get, post},
};
use shield::User;

use crate::routes::*;

#[cfg_attr(feature = "utoipa", derive(utoipa::OpenApi))]
#[cfg_attr(
    feature = "utoipa",
    openapi(paths(providers, sign_in, sign_in_callback, sign_out, user))
)]
pub struct AuthRoutes;

impl AuthRoutes {
    pub fn router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>() -> Router<S> {
        Router::new()
            .route("/providers", get(providers::<U>))
            .route("/sign-in/{methodId}", post(sign_in::<U>))
            .route("/sign-in/{methodId}/{providerId}", post(sign_in::<U>))
            .route("/sign-in/callback/{methodId}", get(sign_in_callback::<U>))
            .route(
                "/sign-in/callback/{methodId}/{providerId}",
                get(sign_in_callback::<U>),
            )
            .route("/sign-out", post(sign_out::<U>))
            .route("/user", get(user::<U>))
    }
}
