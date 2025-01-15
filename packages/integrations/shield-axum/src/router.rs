use axum::{
    routing::{get, post},
    Router,
};
use shield::User;

use crate::routes::{sign_in, sign_in_callback, sign_out, subproviders, user};

pub fn auth_router<U: User + Clone + 'static, S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/subproviders", get(subproviders::<U>))
        .route("/sign-in/:provider_id", post(sign_in::<U>))
        .route("/sign-in/:provider_id/:subprovider_id", post(sign_in::<U>))
        .route("/sign-in/callback/:provider_id", get(sign_in_callback::<U>))
        .route(
            "/sign-in/callback/:provider_id/:subprovider_id",
            get(sign_in_callback::<U>),
        )
        .route("/sign-out", post(sign_out::<U>))
        .route("/user", get(user::<U>))
}
