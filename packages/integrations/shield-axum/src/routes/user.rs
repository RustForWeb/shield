use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use shield::User;

use crate::extract::ExtractUser;

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(as = User))]
#[serde(rename_all = "camelCase")]
struct UserBody {
    id: String,
    name: Option<String>,
}

impl UserBody {
    async fn new<U: User>(user: U) -> Self {
        // TODO: Include email addresses.
        // let email_addresses = user.email_addresses().await;

        Self {
            id: user.id(),
            name: user.name(),
        }
    }
}

#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/user",
        description = "Get the current user account.",
        responses(
            (status = 200, description = "Current user account.", body = UserBody),
            (status = 401, description = "No account signed in."),
            (status = 500, description = "Internal server error."),
        )
    )
)]
pub async fn user<U: User>(ExtractUser(user): ExtractUser<U>) -> Response {
    // TODO: Send JSON error using some util.
    match user {
        Some(user) => Json(UserBody::new(user).await).into_response(),
        None => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}
