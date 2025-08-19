use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Form, Request, Response, Session, ShieldError, SignOutAction, erased_action};
use workos_sdk::WorkOs;

use crate::provider::WorkosProvider;

pub struct WorkosSignOutAction {
    // TODO: Remove expect.
    #[expect(unused)]
    client: Arc<WorkOs>,
}

impl WorkosSignOutAction {
    pub fn new(client: Arc<WorkOs>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider> for WorkosSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn condition(&self, provider: &WorkosProvider, session: Session) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: WorkosProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: sign out
        Ok(Response::Default)
    }
}

erased_action!(WorkosSignOutAction);
