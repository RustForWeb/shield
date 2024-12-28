use serde::{Deserialize, Serialize};
use shield::EmailAddress;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) email_addresses: Vec<EmailAddress>,
}

#[typetag::serde]
impl shield::User for User {
    fn id(&self) -> String {
        self.id.clone()
    }
}
