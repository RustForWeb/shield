use axum::{
    Form,
    extract::{Path, Query},
    response::{IntoResponse, Redirect, Response},
};
use serde_json::Value;
use shield::{Request, ResponseType, User};

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
) -> Result<Response, RouteError> {
    // TODO: Check if this action supports the HTTP method (GET/POST)?

    let response = shield
        .call(
            &action_id,
            &method_id,
            provider_id.as_deref(),
            session,
            Request { query, form_data },
        )
        .await?;

    Ok(match response {
        ResponseType::Default => todo!(),
        ResponseType::Redirect(to) => Redirect::to(&to).into_response(),
        ResponseType::RedirectToAction { action_id } => {
            // TODO: Use actual frontend prefix instead of hardcoded `/auth`.
            Redirect::to(&format!("/auth/{action_id}")).into_response()
        }
    })
}
