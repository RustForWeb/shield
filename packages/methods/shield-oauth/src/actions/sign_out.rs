use async_trait::async_trait;
use shield::{
    Action, Form, Request, Response, SIGN_OUT_ACTION_ID, Session, ShieldError, erased_action,
};

use crate::provider::OauthProvider;

pub struct OauthSignOutAction;

#[async_trait]
impl Action<OauthProvider> for OauthSignOutAction {
    fn id(&self) -> String {
        SIGN_OUT_ACTION_ID.to_owned()
    }

    fn render(&self, _provider: OauthProvider) -> Form {
        Form {
            inputs: vec![],
            attributes: None,
        }
    }

    async fn call(
        &self,
        _provider: OauthProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: OAuth token revocation.

        Ok(Response::Default)
    }
}

erased_action!(OauthSignOutAction);
