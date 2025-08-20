use std::sync::Arc;

use async_trait::async_trait;
use shield::{
    Action, Form, Input, InputType, InputTypeEmail, InputTypeHidden, InputTypePassword,
    InputTypeSubmit, Request, Response, Session, ShieldError, SignUpAction, erased_action,
};

use crate::{client::WorkosClient, provider::WorkosProvider};

pub struct WorkosSignUpAction {
    // TODO: Remove expect.
    #[expect(unused)]
    client: Arc<WorkosClient>,
}

impl WorkosSignUpAction {
    pub fn new(client: Arc<WorkosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider> for WorkosSignUpAction {
    fn id(&self) -> String {
        SignUpAction::id()
    }

    fn name(&self) -> String {
        SignUpAction::name()
    }

    async fn forms(&self, _provider: WorkosProvider) -> Result<Vec<Form>, ShieldError> {
        // TODO: Magic auth and SSO buttons.
        // TODO: Prefill email address.

        Ok(vec![
            Form {
                inputs: vec![
                    Input {
                        name: "email".to_owned(),
                        label: Some("Email address".to_owned()),
                        r#type: InputType::Email(InputTypeEmail {
                            autocomplete: Some("email".to_owned()),
                            placeholder: Some("Email address".to_owned()),
                            required: Some(true),
                            ..Default::default()
                        }),
                        value: None,
                    },
                    Input {
                        name: "password".to_owned(),
                        label: Some("Password".to_owned()),
                        r#type: InputType::Password(InputTypePassword {
                            autocomplete: Some("new-password".to_owned()),
                            placeholder: Some("Password".to_owned()),
                            required: Some(true),
                            ..Default::default()
                        }),
                        value: None,
                    },
                    Input {
                        name: "submit".to_owned(),
                        label: None,
                        r#type: InputType::Submit(InputTypeSubmit::default()),
                        value: Some("Sign up".to_owned()),
                    },
                ],
            },
            Form {
                inputs: vec![
                    Input {
                        name: "email".to_owned(),
                        label: None,
                        r#type: InputType::Hidden(InputTypeHidden {
                            autocomplete: Some("email".to_owned()),
                            required: Some(true),
                        }),
                        value: None,
                    },
                    Input {
                        name: "submit".to_owned(),
                        label: None,
                        r#type: InputType::Submit(InputTypeSubmit::default()),
                        value: Some("Email sign-up code".to_owned()),
                    },
                ],
            },
        ])
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: sign in
        Ok(Response::Default)
    }
}

erased_action!(WorkosSignUpAction);
