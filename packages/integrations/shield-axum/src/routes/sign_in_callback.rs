use axum::extract::{Path, Query};
use serde_json::Value;
use shield::SignInCallbackRequest;

use crate::{
    error::RouteError,
    extract::{ExtractSession, ExtractShield},
    path::AuthPath,
    response::RouteResponse,
};

pub async fn sign_in_callback(
    Path(AuthPath {
        provider_id,
        subprovider_id,
    }): Path<AuthPath>,
    Query(query): Query<Value>,
    ExtractShield(shield): ExtractShield,
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
