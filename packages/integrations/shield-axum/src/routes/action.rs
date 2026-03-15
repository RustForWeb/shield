use axum::{
    Form,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use serde_json::Value;
use shield::{ActionPathParams, MethodActionPathParams, Request, ResponseType, User};

#[cfg(feature = "utoipa")]
use crate::error::ErrorBody;
use crate::{ExtractSession, ExtractShield, RouteError};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        post,
        path = "/{actionId}",
        operation_id = "callAction",
        summary = "Call action",
        description = "Call an action.",
        tags = ["auth"],
        params(
            ActionPathParams
        ),
        responses(
            (status = NO_CONTENT, description = "Success."),
            (status = SEE_OTHER, description = "Redirect."),
            (status = INTERNAL_SERVER_ERROR, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn action<U: User>(
    Path(ActionPathParams { action_id }): Path<ActionPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
    Query(query): Query<Value>,
    Form(form_data): Form<Value>,
) -> Result<Response, RouteError> {
    // TODO: Check if this action supports the HTTP method (GET/POST)?

    let response = shield
        .call(&action_id, session, Request { query, form_data })
        .await?;

    Ok(match response {
        ResponseType::Default => StatusCode::NO_CONTENT.into_response(),
        ResponseType::Redirect(to) => Redirect::to(&to).into_response(),
        ResponseType::RedirectToAction { action_id } => {
            // TODO: Use actual frontend prefix instead of hardcoded `/auth`.
            Redirect::to(&format!("/auth/{action_id}")).into_response()
        }
    })
}

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        post,
        path = "/{actionId}/{methodId}/{providerId}",
        operation_id = "callMethodAction",
        summary = "Call method action",
        description = "Call a method action.",
        tags = ["auth"],
        params(
            MethodActionPathParams
        ),
        responses(
            (status = NO_CONTENT, description = "Success."),
            (status = SEE_OTHER, description = "Redirect."),
            (status = INTERNAL_SERVER_ERROR, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn method_action<U: User>(
    Path(MethodActionPathParams {
        action_id,
        method_id,
        provider_id,
        ..
    }): Path<MethodActionPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
    Query(query): Query<Value>,
    Form(form_data): Form<Value>,
) -> Result<Response, RouteError> {
    // TODO: Check if this action supports the HTTP method (GET/POST)?

    let response = shield
        .call_method(
            &action_id,
            &method_id,
            provider_id.as_deref(),
            session,
            Request { query, form_data },
        )
        .await?;

    Ok(match response {
        ResponseType::Default => StatusCode::NO_CONTENT.into_response(),
        ResponseType::Redirect(to) => Redirect::to(&to).into_response(),
        ResponseType::RedirectToAction { action_id } => {
            // TODO: Use actual frontend prefix instead of hardcoded `/auth`.
            Redirect::to(&format!("/auth/{action_id}")).into_response()
        }
    })
}
