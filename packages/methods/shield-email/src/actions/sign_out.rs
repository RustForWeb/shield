use async_trait::async_trait;
use shield::{
    Action, ActionMethod, Form, MethodSession, Request, Response, ResponseType, SessionAction,
    ShieldError, SignOutAction, erased_action,
};

use crate::provider::EmailProvider;

pub struct EmailSignOutAction;

#[async_trait]
impl Action<EmailProvider, ()> for EmailSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign out with OpenID Connect"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign out with OpenID Connect."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    fn condition(
        &self,
        provider: &EmailProvider,
        session: &MethodSession<()>,
    ) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: EmailProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: EmailProvider,
        _session: &MethodSession<()>,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        Ok(Response::new(ResponseType::Default).session_action(SessionAction::Unauthenticate))
    }
}

erased_action!(EmailSignOutAction);
