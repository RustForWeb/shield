use std::ops::Deref;

use workos::{WorkOs, sso::ClientId};

pub struct WorkosClient {
    client: WorkOs,
    client_id: String,
}

impl WorkosClient {
    pub fn new(client: WorkOs, client_id: &str) -> Self {
        Self {
            client,
            client_id: client_id.to_owned(),
        }
    }

    pub fn client_id(&self) -> ClientId {
        ClientId::from(self.client_id.as_str())
    }
}

impl Deref for WorkosClient {
    type Target = WorkOs;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
