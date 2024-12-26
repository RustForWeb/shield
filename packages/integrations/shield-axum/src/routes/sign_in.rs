use axum::extract::Path;
use shield::{SignInRequest, User};

use crate::{
    error::RouteError,
    extract::{ExtractSession, ExtractShield},
    path::AuthPath,
    response::RouteResponse,
};

pub async fn sign_in<U: User>(
    Path(AuthPath {
        provider_id,
        subprovider_id,
    }): Path<AuthPath>,
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
