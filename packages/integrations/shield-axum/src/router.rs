use axum::{
    routing::{get, post},
    Router,
};
use shield::User;

use crate::routes::*;

#[cfg_attr(feature = "utoipa", derive(utoipa::OpenApi))]
#[cfg_attr(
    feature = "utoipa",
    openapi(paths(subproviders, sign_in, sign_in_callback, sign_out, user))
)]
pub struct AuthRoutes;

impl AuthRoutes {
    pub fn router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>() -> Router<S> {
        Router::new()
            .route("/subproviders", get(subproviders::<U>))
            .route("/sign-in/:providerId", post(sign_in::<U>))
            .route("/sign-in/:providerId/:subproviderId", post(sign_in::<U>))
            .route("/sign-in/callback/:providerId", get(sign_in_callback::<U>))
            .route(
                "/sign-in/callback/:providerId/:subproviderId",
                get(sign_in_callback::<U>),
            )
            .route("/sign-out", post(sign_out::<U>))
            .route("/user", get(user::<U>))
    }
}
