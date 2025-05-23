use chrono::{DateTime, FixedOffset};

#[derive(Clone, Debug)]
pub struct OauthConnection {
    pub id: String,
    pub identifier: String,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expired_at: Option<DateTime<FixedOffset>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct CreateOauthConnection {
    pub identifier: String,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expired_at: Option<DateTime<FixedOffset>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct UpdateOauthConnection {
    pub id: String,
    pub token_type: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<Option<String>>,
    pub expired_at: Option<Option<DateTime<FixedOffset>>>,
    pub scopes: Option<Option<Vec<String>>>,
}
