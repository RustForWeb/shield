use crate::{MethodSession, Provider, ShieldError};

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

    pub fn condition<P: Provider, S>(
        _provider: &P,
        _session: &MethodSession<S>,
    ) -> Result<bool, ShieldError> {
        Ok(true)
    }
}
