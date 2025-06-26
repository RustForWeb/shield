use chrono::{DateTime, FixedOffset};
use secrecy::SecretString;

#[derive(Clone, Debug)]
pub struct OauthConnection {
    pub id: String,
    pub identifier: String,
    pub token_type: String,
    pub access_token: SecretString,
    pub refresh_token: Option<SecretString>,
    pub expired_at: Option<DateTime<FixedOffset>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct CreateOauthConnection {
    pub identifier: String,
    pub token_type: String,
    pub access_token: SecretString,
    pub refresh_token: Option<SecretString>,
    pub expired_at: Option<DateTime<FixedOffset>>,
    pub scopes: Option<Vec<String>>,
    pub provider_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct UpdateOauthConnection {
    pub id: String,
    pub token_type: Option<String>,
    pub access_token: Option<SecretString>,
    pub refresh_token: Option<Option<SecretString>>,
    pub expired_at: Option<Option<DateTime<FixedOffset>>>,
    pub scopes: Option<Option<Vec<String>>>,
}
