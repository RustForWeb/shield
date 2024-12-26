use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct OidcConnection {
    pub id: String,
    pub identifier: String,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expired_at: Option<DateTime<Utc>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct CreateOidcConnection {
    pub identifier: String,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expired_at: Option<DateTime<Utc>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct UpdateOidcConnection {
    pub id: String,
    pub token_type: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expired_at: Option<DateTime<Utc>>,
    pub scopes: Option<Vec<String>>,
}
