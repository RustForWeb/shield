#[cfg(feature = "actix")]
pub use leptos_actix::extract;
#[cfg(feature = "axum")]
pub use leptos_axum::extract;

#[cfg(not(any(feature = "actix", feature = "axum")))]
use leptos::prelude::ServerFnError;

#[cfg(not(any(feature = "actix", feature = "axum")))]
#[expect(dead_code)]
pub fn extract<T>() -> Result<T, ServerFnError> {
    Err(ServerFnError::ServerError(
        "Missing `extract` function. Enable the either the `actix` or `axum` feature.".to_owned(),
    ))
}
