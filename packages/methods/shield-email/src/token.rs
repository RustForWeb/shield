use chrono::{DateTime, FixedOffset};
use secrecy::{ExposeSecret, SecretString};
use sha3::{Digest, Sha3_256};

#[derive(Clone, Debug)]
pub struct EmailAuthToken {
    pub id: String,
    pub email: String,
    pub token: String,
    pub expired_at: DateTime<FixedOffset>,
}

#[derive(Clone, Debug)]
pub struct CreateEmailAuthToken {
    pub email: String,
    pub token: String,
    pub expired_at: DateTime<FixedOffset>,
}

pub(crate) fn hash_token(token: &str, secret: &SecretString) -> String {
    hex::encode(
        Sha3_256::new()
            .chain_update(token)
            .chain_update(secret.expose_secret())
            .finalize(),
    )
}
