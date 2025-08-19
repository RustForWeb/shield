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

    fn forms(&self, provider: OauthProvider) -> Vec<Form> {
        SignOutAction::forms(provider)
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
