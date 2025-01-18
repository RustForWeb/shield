use axum::extract::{Path, Query};
use serde_json::Value;
use shield::{SignInCallbackRequest, User};

use crate::{
    error::RouteError,
    extract::{ExtractSession, ExtractShield},
    path::AuthPathParams,
    response::RouteResponse,
};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        post,
        path = "/sign-in/callback/{providerId}/{subproviderId}",
        operation_id = "signInCallback",
        description = "Callback after signing in with authentication provider.",
        params(
            AuthPathParams,
        ),
        responses(
            (status = 200, description = "Successfully signed in."),
            (status = 500, description = "Internal server error."),
        )
    )
)]
pub async fn sign_in_callback<U: User>(
    Path(AuthPathParams {
        provider_id,
        subprovider_id,
    }): Path<AuthPathParams>,
    Query(query): Query<Value>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
) -> Result<RouteResponse, RouteError> {
    let response = shield
        .sign_in_callback(
            SignInCallbackRequest {
                provider_id,
                subprovider_id,
                query: Some(query),
                data: None,
            },
            session,
        )
        .await?;

    Ok(response.into())
}
