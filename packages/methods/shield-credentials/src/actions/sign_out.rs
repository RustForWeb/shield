use async_trait::async_trait;
use shield::{Action, Form, Request, Response, Session, ShieldError, SignOutAction, erased_action};

use crate::provider::CredentialsProvider;

pub struct CredentialsSignOutAction;

#[async_trait]
impl Action<CredentialsProvider> for CredentialsSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn condition(
        &self,
        provider: &CredentialsProvider,
        session: Session,
    ) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: CredentialsProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: CredentialsProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: sign out
        Ok(Response::Default)
    }
}

erased_action!(CredentialsSignOutAction);
