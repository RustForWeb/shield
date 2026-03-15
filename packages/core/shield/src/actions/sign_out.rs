use async_trait::async_trait;

use crate::{
    action::Action,
    error::ShieldError,
    form::{Form, Input, InputType, InputTypeSubmit, InputValue},
    request::{Request, RequestMethod},
    response::{Response, ResponseType},
    session::{BaseSession, SessionAction},
};

const ACTION_ID: &str = "sign-out";
const ACTION_NAME: &str = "Sign out";

pub struct SignOutAction;

#[async_trait]
impl Action for SignOutAction {
    fn id(&self) -> &'static str {
        ACTION_ID
    }

    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign out"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign out."
    }

    fn method(&self) -> RequestMethod {
        RequestMethod::Post
    }

    async fn forms(&self) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![Form {
            inputs: vec![Input {
                name: "submit".to_owned(),
                label: None,
                r#type: InputType::Submit(InputTypeSubmit {}),
                value: Some(InputValue::String {
                    value: self.name().to_owned(),
                }),
                addon_start: None,
                addon_end: None,
            }],
        }])
    }

    async fn call(
        &self,
        _session: &BaseSession,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        Ok(Response::new(ResponseType::Default).session_action(SessionAction::Unauthenticate))
    }
}
