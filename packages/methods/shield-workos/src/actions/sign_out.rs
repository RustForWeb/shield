use std::sync::Arc;

use async_trait::async_trait;
use shield::{
    Action, Form, MethodSession, Request, Response, ResponseType, SessionAction, ShieldError,
    SignOutAction, erased_action,
};

use crate::{client::WorkosClient, provider::WorkosProvider};

pub struct WorkosSignOutAction {
    // TODO: Remove expect.
    #[expect(unused)]
    client: Arc<WorkosClient>,
}

impl WorkosSignOutAction {
    pub fn new(client: Arc<WorkosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider, ()> for WorkosSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn condition(
        &self,
        provider: &WorkosProvider,
        session: &MethodSession<()>,
    ) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: WorkosProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: &MethodSession<()>,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: Handle WorkOS sign out.

        Ok(Response::new(ResponseType::Default).session_action(SessionAction::Unauthenticate))
    }
}

erased_action!(WorkosSignOutAction);
