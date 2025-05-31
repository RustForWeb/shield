use std::sync::Arc;

use serde::de::DeserializeOwned;
use shield::{Form, Provider, User};

use crate::{CREDENTIALS_METHOD_ID, Credentials};

pub struct CredentialsProvider<U: User, D: DeserializeOwned> {
    credentials: Arc<dyn Credentials<U, D>>,
}

impl<U: User, D: DeserializeOwned> CredentialsProvider<U, D> {
    pub(crate) fn new(credentials: Arc<dyn Credentials<U, D>>) -> Self {
        Self { credentials }
    }
}

impl<U: User, D: DeserializeOwned> Provider for CredentialsProvider<U, D> {
    fn method_id(&self) -> String {
        CREDENTIALS_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "Credentials".to_owned()
    }

    fn icon_url(&self) -> Option<String> {
        None
    }

    fn form(&self) -> Option<Form> {
        Some(self.credentials.form())
    }
}
