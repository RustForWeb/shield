use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
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

pub trait User: Send + Sync {
    fn id(&self) -> String;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::User;

    #[derive(Clone, Debug)]
    pub struct TestUser {
        id: String,
    }

    impl User for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }
    }
}
