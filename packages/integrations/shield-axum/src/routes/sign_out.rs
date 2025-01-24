use shield::User;

use crate::{
    error::{ErrorBody, RouteError},
    extract::{ExtractSession, ExtractShield},
    response::RouteResponse,
};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        post,
        path = "/sign-out",
        operation_id = "signOut",
        description = "Sign out of the current account.",
        responses(
            (status = 201, description = "Successfully signed out."),
            (status = 400, description = "Bad request.", body = ErrorBody),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn sign_out<U: User>(
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
) -> Result<RouteResponse, RouteError> {
    let response = shield.sign_out(session).await?;

    Ok(response.into())
}
