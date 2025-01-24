use axum::Json;
use serde::{Deserialize, Serialize};
use shield::{EmailAddress, ShieldError, User};

use crate::{
    error::{ErrorBody, RouteError},
    extract::ExtractUser,
};

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(as = User))]
#[serde(rename_all = "camelCase")]
pub struct UserBody {
    id: String,
    name: Option<String>,
    email_addresses: Vec<EmailAddress>,
}

impl UserBody {
    async fn new<U: User>(user: U) -> Result<Self, ShieldError> {
        let email_addresses = user.email_addresses().await?;

        Ok(Self {
            id: user.id(),
            name: user.name(),
            email_addresses,
        })
    }
}

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/user",
        operation_id = "getUser",
        description = "Get the current user account.",
        responses(
            (status = 200, description = "Current user account.", body = UserBody),
            (status = 401, description = "No account signed in.", body = ErrorBody),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn user<U: User>(
    ExtractUser(user): ExtractUser<U>,
) -> Result<Json<UserBody>, RouteError> {
    let user = user.ok_or(ShieldError::Unauthorized)?;

    Ok(Json(UserBody::new(user).await?))
}
