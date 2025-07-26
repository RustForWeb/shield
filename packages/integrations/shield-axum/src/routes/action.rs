use axum::{
    Form,
    extract::{Path, Query},
};
use serde_json::Value;
use shield::{Request, User};

use crate::{ExtractSession, ExtractShield, RouteError, path::ActionPathParams};

pub async fn action<U: User>(
    Path(ActionPathParams {
        method_id,
        action_id,
        provider_id,
        ..
    }): Path<ActionPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
    Query(query): Query<Value>,
    Form(form_data): Form<Value>,
) -> Result<(), RouteError> {
    // TODO: Check if this action supports the HTTP method (GET/POST)?

    shield
        .call(
            &method_id,
            &action_id,
            provider_id.as_deref(),
            session,
            Request { query, form_data },
        )
        .await?;

    Ok(())
}
