use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Method, ShieldError, erased_method};
use workos::{ApiKey, WorkOs};

use crate::{
    actions::{WorkosIndexAction, WorkosSignInAction, WorkosSignOutAction, WorkosSignUpAction},
    client::WorkosClient,
    options::WorkosOptions,
    provider::WorkosProvider,
};

pub const WORKOS_METHOD_ID: &str = "workos";

pub struct WorkosMethod {
    options: WorkosOptions,
    client: Arc<WorkosClient>,
}

impl WorkosMethod {
    pub fn new(client: WorkOs, client_id: &str, options: WorkosOptions) -> Self {
        Self {
            options,
            client: Arc::new(WorkosClient::new(client, client_id)),
        }
    }

    pub fn from_api_key(api_key: &str, client_id: &str, options: WorkosOptions) -> Self {
        Self::new(WorkOs::new(&ApiKey::from(api_key)), client_id, options)
    }

    pub fn with_options(mut self, options: WorkosOptions) -> Self {
        self.options = options;
        self
    }
}

#[async_trait]
impl Method<WorkosProvider> for WorkosMethod {
    fn id(&self) -> String {
        WORKOS_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn Action<WorkosProvider>>> {
        vec![
            Box::new(WorkosIndexAction::new(
                self.options.clone(),
                self.client.clone(),
            )),
            Box::new(WorkosSignInAction::new(self.client.clone())),
            Box::new(WorkosSignUpAction::new(self.client.clone())),
            Box::new(WorkosSignOutAction::new(self.client.clone())),
        ]
    }

    async fn providers(&self) -> Result<Vec<WorkosProvider>, ShieldError> {
        Ok(vec![WorkosProvider])
    }
}

erased_method!(WorkosMethod);
