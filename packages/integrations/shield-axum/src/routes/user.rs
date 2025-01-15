use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use shield::User;

use crate::extract::ExtractUser;

pub async fn user<U: User>(ExtractUser(user): ExtractUser<U>) -> Response {
    // TODO: Send JSON error using some util.
    match user {
        Some(user) => {
            // TODO: Include email addresses.
            // let email_addresses = user.email_addresses().await;

            Json(json!({
                "id": user.id(),
                "name": user.name(),
            }))
            .into_response()
        }
        None => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}
