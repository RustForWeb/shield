use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shield::{EmailAddress, StorageError};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) email_addresses: Vec<EmailAddress>,
}

#[async_trait]
impl shield::User for User {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    async fn email_addresses(&self) -> Result<Vec<EmailAddress>, StorageError> {
        Ok(self.email_addresses.clone())
    }

    fn additional(&self) -> Option<impl Serialize> {
        None::<()>
    }
}
