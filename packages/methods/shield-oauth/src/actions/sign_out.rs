use async_trait::async_trait;
use shield::{
    Action, ActionMethod, Form, MethodSession, Request, Response, ResponseType, SessionAction,
    ShieldError, SignOutAction, erased_action,
};

use crate::{provider::OauthProvider, session::OauthSession};

pub struct OauthSignOutAction;

#[async_trait]
impl Action<OauthProvider, OauthSession> for OauthSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign out with OAuth"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign out with OAuth."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    fn condition(
        &self,
        provider: &OauthProvider,
        session: &MethodSession<OauthSession>,
    ) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: OauthProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: OauthProvider,
        _session: &MethodSession<OauthSession>,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: OAuth token revocation.

        Ok(Response::new(ResponseType::Default).session_action(SessionAction::Unauthenticate))
    }
}

erased_action!(OauthSignOutAction);
