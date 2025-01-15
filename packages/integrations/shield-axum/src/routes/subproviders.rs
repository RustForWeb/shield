use axum::Json;
use shield::{SubproviderVisualisation, User};

use crate::{error::RouteError, extract::ExtractShield};

pub async fn subproviders<U: User>(
    ExtractShield(shield): ExtractShield<U>,
) -> Result<Json<Vec<SubproviderVisualisation>>, RouteError> {
    Ok(Json(shield.subprovider_visualisations().await?))
}
