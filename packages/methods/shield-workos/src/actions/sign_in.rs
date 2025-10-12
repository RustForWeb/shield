use std::sync::Arc;

use async_trait::async_trait;
use shield::{
    Action, ActionMethod, Form, Input, InputType, InputTypeEmail, InputTypeHidden,
    InputTypePassword, InputTypeSubmit, MethodSession, Request, Response, ResponseType,
    ShieldError, SignInAction, erased_action,
};

use crate::{client::WorkosClient, provider::WorkosProvider};

pub struct WorkosSignInAction {
    // TODO: Remove expect.
    #[expect(unused)]
    client: Arc<WorkosClient>,
}

impl WorkosSignInAction {
    pub fn new(client: Arc<WorkosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider, ()> for WorkosSignInAction {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in with WorkOS"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in with WorkOS."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
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
        ])
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: &MethodSession<()>,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: sign in
        Ok(Response::new(ResponseType::Default))
    }
}

erased_action!(WorkosSignInAction);
