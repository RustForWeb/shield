use async_trait::async_trait;
use shield::{Action, Form, Request, Response, Session, ShieldError, SignOutAction, erased_action};

use crate::provider::OauthProvider;

pub struct OauthSignOutAction;

#[async_trait]
impl Action<OauthProvider> for OauthSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn condition(&self, provider: &OauthProvider, session: Session) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    async fn forms(&self, provider: OauthProvider) -> Result<Vec<Form>, ShieldError> {
        SignOutAction::forms(provider).await
    }

    async fn call(
        &self,
        _provider: OauthProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: OAuth token revocation.
        // TODO: Sign out.

        Ok(Response::Default)
    }
}

erased_action!(OauthSignOutAction);
