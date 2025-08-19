use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Method, ShieldError, erased_method};
use workos_sdk::{ApiKey, WorkOs};

use crate::{
    actions::{WorkosIndexAction, WorkosSignInAction, WorkosSignOutAction, WorkosSignUpAction},
    options::WorkosOptions,
    provider::WorkosProvider,
};

pub const WORKOS_METHOD_ID: &str = "workos";

pub struct WorkosMethod {
    options: WorkosOptions,
    client: Arc<WorkOs>,
}

impl WorkosMethod {
    pub fn new(client: WorkOs) -> Self {
        Self {
            options: WorkosOptions::default(),
            client: Arc::new(client),
        }
    }

    pub fn from_api_key(api_key: &str) -> Self {
        Self::new(WorkOs::new(&ApiKey::from(api_key)))
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
