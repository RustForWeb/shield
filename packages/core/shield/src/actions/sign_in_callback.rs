use crate::{Provider, Session, ShieldError};

const ACTION_ID: &str = "sign-in-callback";
const ACTION_NAME: &str = "Sign in callback";

pub struct SignInCallbackAction;

impl SignInCallbackAction {
    pub fn id() -> String {
        ACTION_ID.to_owned()
    }

    pub fn name() -> String {
        ACTION_NAME.to_owned()
    }

    pub fn condition<P: Provider>(_provider: &P, _session: Session) -> Result<bool, ShieldError> {
        Ok(false)
    }
}
