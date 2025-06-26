use axum::{
    Form,
    extract::{Path, Query},
};
use serde_json::Value;
use shield::{ActionError, MethodError, ProviderError, Request, ShieldError, User};

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
    let method = shield
        .method_by_id(&method_id)
        .ok_or(ShieldError::Method(MethodError::NotFound(method_id)))?;

    let action = method
        .erased_action_by_id(&action_id)
        .ok_or(ShieldError::Action(ActionError::NotFound(action_id)))?;

    // TODO: Check if this action supports the HTTP method (GET/POST).

    let provider = method
        .erased_provider_by_id(provider_id.as_deref())
        .await?
        .ok_or(ShieldError::Provider(ProviderError::NotFound(provider_id)))?;

    action
        .erased_call(provider, session, Request { query, form_data })
        .await?;

    Ok(())
}
