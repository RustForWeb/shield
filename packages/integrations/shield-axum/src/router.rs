use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{sign_in, sign_in_callback, sign_out};

pub fn auth_router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/sign-in/:provider_id", post(sign_in))
        .route("/sign-in/:provider_id/:subprovider_id", post(sign_in))
        .route("/sign-in/callback/:provider_id", get(sign_in_callback))
        .route(
            "/sign-in/callback/:provider_id/:subprovider_id",
            get(sign_in_callback),
        )
        .route("/sign-out", post(sign_out))
}
