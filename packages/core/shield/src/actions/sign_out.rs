use crate::{
    Form, Input, InputType, InputTypeSubmit, Provider, Session, SessionError, ShieldError,
};

const ACTION_ID: &str = "sign-out";
const ACTION_NAME: &str = "Sign out";

pub struct SignOutAction;

impl SignOutAction {
    pub fn id() -> String {
        ACTION_ID.to_owned()
    }

    pub fn name() -> String {
        ACTION_NAME.to_owned()
    }

    pub fn condition<P: Provider>(provider: &P, session: Session) -> Result<bool, ShieldError> {
        let session_data = session.data();
        let session_data = session_data
            .lock()
            .map_err(|err| SessionError::Lock(err.to_string()))?;

        Ok(session_data
            .authentication
            .as_ref()
            .is_some_and(|authentication| {
                authentication.method_id == provider.method_id()
                    && authentication.provider_id == provider.id()
            }))
    }

    pub fn form<P: Provider>(_provider: P) -> Form {
        Form {
            inputs: vec![Input {
                name: "submit".to_owned(),
                label: None,
                r#type: InputType::Submit(InputTypeSubmit {}),
                value: Some(Self::name()),
            }],
        }
    }
}
