use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shield::{
    Action, ActionMethod, Form, Input, InputType, InputTypeText, MethodSession, Request, Response,
    ResponseType, SessionAction, ShieldError, SignInAction, Storage, User, erased_action,
};

use crate::provider::DummyProvider;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInData {
    pub user_id: String,
}

pub struct DummySignInAction<U: User> {
    storage: Arc<dyn Storage<U>>,
}

impl<U: User> DummySignInAction<U> {
    pub fn new(storage: Arc<dyn Storage<U>>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<U: User + 'static> Action<DummyProvider, ()> for DummySignInAction<U> {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn openapi_summary(&self) -> &'static str {
        "Sign in with dummy"
    }

    fn openapi_description(&self) -> &'static str {
        "Sign in with dummy."
    }

    fn method(&self) -> ActionMethod {
        ActionMethod::Post
    }

    async fn forms(&self, _provider: DummyProvider) -> Result<Vec<Form>, ShieldError> {
        Ok(vec![Form {
            inputs: vec![Input {
                name: "userId".to_owned(),
                label: Some("User ID".to_owned()),
                r#type: InputType::Text(InputTypeText {
                    placeholder: Some("User ID".to_owned()),
                    required: Some(true),
                    ..Default::default()
                }),
                value: None,
                addon_start: None,
                addon_end: None,
            }],
        }])
    }

    async fn call(
        &self,
        _provider: DummyProvider,
        _session: &MethodSession<()>,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let data = serde_json::from_value::<SignInData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let user = self
            .storage
            .user_by_id(&data.user_id)
            .await?
            .ok_or_else(|| ShieldError::Validation("User not found.".to_owned()))?;

        Ok(Response::new(ResponseType::Default).session_action(SessionAction::authenticate(user)))
    }
}

erased_action!(DummySignInAction, <U: User>);
