use async_trait::async_trait;
use shield::{Method, MethodAction, ShieldError, erased_method};
use workos::Client;

use crate::{
    actions::{WorkosIndexAction, WorkosSignInAction, WorkosSignUpAction},
    options::WorkosOptions,
    provider::WorkosProvider,
};

// TODO: Add hook for WorkOS sign out.

pub const WORKOS_METHOD_ID: &str = "workos";

pub struct WorkosMethod {
    options: WorkosOptions,
    client: Client,
}

impl WorkosMethod {
    pub fn new(client: Client, options: WorkosOptions) -> Self {
        Self { options, client }
    }

    pub fn from_api_key(api_key: &str, client_id: &str, options: WorkosOptions) -> Self {
        Self::new(
            Client::builder()
                .api_key(api_key)
                .client_id(client_id)
                .build(),
            options,
        )
    }

    pub fn with_options(mut self, options: WorkosOptions) -> Self {
        self.options = options;
        self
    }
}

#[async_trait]
impl Method for WorkosMethod {
    type Provider = WorkosProvider;
    type Session = ();

    fn id(&self) -> String {
        WORKOS_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn MethodAction<Self::Provider, Self::Session>>> {
        vec![
            Box::new(WorkosIndexAction::new(
                self.options.clone(),
                self.client.clone(),
            )),
            Box::new(WorkosSignInAction::new(self.client.clone())),
            Box::new(WorkosSignUpAction::new(self.client.clone())),
        ]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(vec![WorkosProvider])
    }
}

erased_method!(WorkosMethod);
