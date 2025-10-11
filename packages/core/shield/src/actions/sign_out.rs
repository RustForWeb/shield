use crate::{Form, Input, InputType, InputTypeSubmit, MethodSession, Provider, ShieldError};

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

    pub fn condition<P: Provider, S>(
        provider: &P,
        session: &MethodSession<S>,
    ) -> Result<bool, ShieldError> {
        Ok(session
            .base
            .authentication
            .as_ref()
            .is_some_and(|authentication| {
                authentication.method_id == provider.method_id()
                    && authentication.provider_id == provider.id()
            }))
    }

    pub async fn forms<P: Provider>(_provider: P) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![Form {
            inputs: vec![Input {
                name: "submit".to_owned(),
                label: None,
                r#type: InputType::Submit(InputTypeSubmit {}),
                value: Some(Self::name()),
            }],
        }])
    }
}
