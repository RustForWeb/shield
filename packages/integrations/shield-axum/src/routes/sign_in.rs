use axum::extract::Path;
use shield::{SignInRequest, User};

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
        path = "/sign-in/{providerId}/{subproviderId}",
        operation_id = "signIn",
        description = "Sign in to an account with the specified authentication provider.",
        params(
            AuthPathParams,
        ),
        responses(
            (status = 200, description = "Successfully signed in."),
            (status = 303, description = "Redirect to authentication provider for sign in."),
            (status = 500, description = "Internal server error."),
        )
    )
)]
pub async fn sign_in<U: User>(
    Path(AuthPathParams {
        provider_id,
        subprovider_id,
    }): Path<AuthPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
) -> Result<RouteResponse, RouteError> {
    let response = shield
        .sign_in(
            SignInRequest {
                provider_id,
                subprovider_id,
                data: None,
                form_data: None,
            },
            session,
        )
        .await?;

    Ok(response.into())
}
