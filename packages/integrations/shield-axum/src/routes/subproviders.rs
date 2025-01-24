use axum::Json;
use shield::{SubproviderVisualisation, User};

use crate::{
    error::{ErrorBody, RouteError},
    extract::ExtractShield,
};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/subproviders",
        operation_id = "getSubproviders",
        description = "Get a list of authentication subproviders.",
        responses(
            (status = 200, description = "List of authentication subproviders.", body = Vec<SubproviderVisualisation>),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn subproviders<U: User>(
    ExtractShield(shield): ExtractShield<U>,
) -> Result<Json<Vec<SubproviderVisualisation>>, RouteError> {
    Ok(Json(shield.subprovider_visualisations().await?))
}
