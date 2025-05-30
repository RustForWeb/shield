use axum::{Form, extract::Path};
use serde::{Deserialize, Serialize};
use shield::{SignInRequest, User};

use crate::{
    error::{ErrorBody, RouteError},
    extract::{ExtractSession, ExtractShield},
    path::AuthPathParams,
    response::RouteResponse,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SignInData {
    redirect_url: Option<String>,
}

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        post,
        path = "/sign-in/{methodId}/{providerId}",
        operation_id = "signIn",
        description = "Sign in to an account with the specified authentication provider.",
        params(
            AuthPathParams,
        ),
        request_body = SignInData,
        responses(
            (status = 200, description = "Successfully signed in."),
            (status = 303, description = "Redirect to authentication provider for sign in."),
            (status = 400, description = "Bad request.", body = ErrorBody),
            (status = 404, description = "Not found.", body = ErrorBody),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn sign_in<U: User>(
    Path(AuthPathParams {
        method_id,
        provider_id,
    }): Path<AuthPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
    Form(data): Form<SignInData>,
) -> Result<RouteResponse, RouteError> {
    let response = shield
        .sign_in(
            SignInRequest {
                method_id,
                provider_id,
                redirect_url: data.redirect_url,
                data: None,
                form_data: None,
            },
            session,
        )
        .await?;

    Ok(response.into())
}
