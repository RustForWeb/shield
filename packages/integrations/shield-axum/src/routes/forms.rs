use axum::{Json, extract::Path};
use shield::{ActionForms, User};

use crate::{
    ExtractSession, ExtractShield, RouteError, error::ErrorBody, path::ActionFormsPathParams,
};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/forms/{actionId}",
        operation_id = "getActionForms",
        summary = "Get action forms",
        description = "Get action forms.",
        tags = ["auth"],
        params(
            ActionFormsPathParams
        ),
        responses(
            (status = 200, description = "The action forms.", body = ActionForms),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn forms<U: User>(
    Path(ActionFormsPathParams { action_id, .. }): Path<ActionFormsPathParams>,
    ExtractShield(shield): ExtractShield<U>,
    ExtractSession(session): ExtractSession,
) -> Result<Json<ActionForms>, RouteError> {
    let forms = shield.action_forms(&action_id, session).await?;

    Ok(Json(forms))
}
