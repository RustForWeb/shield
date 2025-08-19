use std::sync::Arc;

use async_trait::async_trait;
use shield::{
    Action, Form, Input, InputType, InputTypeEmail, InputTypeHidden, InputTypePassword,
    InputTypeSubmit, Request, Response, Session, ShieldError, SignInAction, erased_action,
};
use workos_sdk::WorkOs;

use crate::provider::WorkosProvider;

pub struct WorkosSignInAction {
    // TODO: Remove expect.
    #[expect(unused)]
    client: Arc<WorkOs>,
}

impl WorkosSignInAction {
    pub fn new(client: Arc<WorkOs>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider> for WorkosSignInAction {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn forms(&self, _provider: WorkosProvider) -> Vec<Form> {
        // TODO: Magic auth and SSO buttons.
        // TODO: Prefill email address.

        vec![
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
                            autocomplete: Some("current-password".to_owned()),
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
                        value: Some("Sign in".to_owned()),
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
                        value: Some("Email sign-in code".to_owned()),
                    },
                ],
            },
        ]
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

erased_action!(WorkosSignInAction);
