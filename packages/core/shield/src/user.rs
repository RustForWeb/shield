use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub trait User: Debug + Send + Sync {
    fn id(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct CreateUser {
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UpdateUser {
    pub id: String,
    pub name: Option<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailAddress {
    pub id: String,
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_token_expired_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct CreateEmailAddress {
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_token_expired_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct UpdateEmailAddress {
    pub id: String,
    pub is_primary: Option<bool>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<Option<String>>,
    pub verification_token_expired_at: Option<Option<DateTime<Utc>>>,
    pub verified_at: Option<Option<DateTime<Utc>>>,
}

#[cfg(test)]
pub(crate) mod tests {
    use serde::{Deserialize, Serialize};

    use super::User;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TestUser {
        id: String,
    }

    impl User for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }
    }
}
