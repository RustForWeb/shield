use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use shield::User as _;
#[cfg(feature = "server")]
use shield_dioxus_axum::ExtractUser;
#[cfg(feature = "server")]
use shield_memory::User as ShieldUser;

#[derive(Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email_addresses: Vec<String>,
}

#[get("/api/user", user: ExtractUser<ShieldUser>)]
pub async fn user() -> Result<Option<User>, ServerFnError> {
    let ExtractUser(user) = user;

    Ok(if let Some(user) = user {
        Some(User {
            id: user.id(),
            name: user.name(),
            email_addresses: user
                .email_addresses()
                .await
                .context("failed to get user email addresses")?
                .into_iter()
                .map(|email_address| email_address.email)
                .collect(),
        })
    } else {
        None
    })
}
