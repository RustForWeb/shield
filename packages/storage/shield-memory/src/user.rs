use shield::EmailAddress;

#[derive(Clone, Debug)]
pub struct User {
    id: String,
    pub(crate) email_addresses: Vec<EmailAddress>,
}

impl shield::User for User {
    fn id(&self) -> String {
        self.id.clone()
    }
}
