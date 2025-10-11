use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shield::{ConfigurationError, EmailAddress, ShieldError, User};

use crate::{RouteError, error::ErrorBody, extract::UserRequired};

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(as = User))]
#[serde(rename_all = "camelCase")]
pub struct UserBody {
    id: String,
    name: Option<String>,
    email_addresses: Vec<EmailAddress>,
    additional: Value,
}

impl UserBody {
    async fn new<U: User>(user: U) -> Result<Self, ShieldError> {
        let email_addresses = user.email_addresses().await?;

        Ok(Self {
            id: user.id(),
            name: user.name(),
            email_addresses,
            additional: serde_json::to_value(user.additional()).map_err(|err| {
                ConfigurationError::Invalid(format!(
                    "additional user data is not serializable: {err}"
                ))
            })?,
        })
    }
}

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/user",
        operation_id = "getCurrentUser",
        summary = "Get current user",
        description = "Get the current user account.",
        tags = ["auth"],
        responses(
            (status = 200, description = "The current user account.", body = UserBody),
            (status = 401, description = "No account signed in.", body = ErrorBody),
            (status = 500, description = "Internal server error.", body = ErrorBody),
        )
    )
)]
pub async fn user<U: User>(
    UserRequired(user): UserRequired<U>,
) -> Result<Json<UserBody>, RouteError> {
    Ok(Json(UserBody::new(user).await?))
}
