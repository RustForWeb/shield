use std::fmt::Debug;

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::error::StorageError;

#[async_trait]
pub trait User: Debug + Send + Sync {
    fn id(&self) -> String;

    fn name(&self) -> Option<String>;

    async fn email_addresses(&self) -> Result<Vec<EmailAddress>, StorageError>;

    fn additional(&self) -> Option<impl Serialize>;
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    pub id: String,
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    #[serde(skip)]
    pub verification_token: Option<String>,
    #[serde(skip)]
    pub verification_token_expired_at: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub verified_at: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct CreateEmailAddress {
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_token_expired_at: Option<DateTime<FixedOffset>>,
    pub verified_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug)]
pub struct UpdateEmailAddress {
    pub id: String,
    pub is_primary: Option<bool>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<Option<String>>,
    pub verification_token_expired_at: Option<Option<DateTime<FixedOffset>>>,
    pub verified_at: Option<Option<DateTime<FixedOffset>>>,
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};

    use crate::StorageError;

    use super::{EmailAddress, User};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TestUser {
        id: String,
        name: Option<String>,
    }

    #[async_trait]
    impl User for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }

        fn name(&self) -> Option<String> {
            self.name.clone()
        }

        async fn email_addresses(&self) -> Result<Vec<EmailAddress>, StorageError> {
            Ok(vec![])
        }

        fn additional(&self) -> Option<impl Serialize> {
            None::<()>
        }
    }
}
