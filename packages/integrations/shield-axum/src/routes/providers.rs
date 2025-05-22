use axum::Json;
use shield::{ProviderVisualisation, User};

use crate::{
    error::{ErrorBody, RouteError},
    extract::ExtractShield,
};

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/providers",
        operation_id = "getProviders",
        description = "Get a list of authentication providers.",
        responses(
            (status = 200, description = "List of authentication providers.", body = Vec<ProviderVisualisation>),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn providers<U: User>(
    ExtractShield(shield): ExtractShield<U>,
) -> Result<Json<Vec<ProviderVisualisation>>, RouteError> {
    Ok(Json(shield.provider_visualisations().await?))
}
