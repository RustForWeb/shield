use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shield::{
    Action, Form, Input, InputType, InputTypeEmail, Request, Response, Session, ShieldError,
    erased_action,
};
use tracing::info;
use workos_sdk::{
    PaginationParams, WorkOs,
    user_management::{ListUsers, ListUsersParams},
};

use crate::provider::WorkosProvider;

// TODO: Make a special case for an index action reachable at the `/auth` root URL.

const ACTION_ID: &str = "index";
const ACTION_NAME: &str = "Index";

#[derive(Debug, Deserialize)]
pub struct EmailData {
    pub email: String,
}

pub struct WorkosIndexAction {
    client: Arc<WorkOs>,
}

impl WorkosIndexAction {
    pub fn new(client: Arc<WorkOs>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Action<WorkosProvider> for WorkosIndexAction {
    fn id(&self) -> String {
        ACTION_ID.to_owned()
    }

    fn name(&self) -> String {
        ACTION_NAME.to_owned()
    }

    fn forms(&self, _provider: WorkosProvider) -> Vec<Form> {
        // TODO: SSO buttons.

        vec![Form {
            inputs: vec![Input {
                name: "email".to_owned(),
                label: Some("Email address".to_owned()),
                r#type: InputType::Email(InputTypeEmail {
                    autocomplete: Some("email".to_owned()),
                    placeholder: Some("Email address".to_owned()),
                    required: Some(true),
                    ..Default::default()
                }),
                value: None,
            }],
        }]
    }

    async fn call(
        &self,
        _provider: WorkosProvider,
        _session: Session,
        request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: Check email address and redirect to sign-in/sign-up action with prefilled email address.
        // TODO: Only check if enabled in options.

        let data = serde_json::from_value::<EmailData>(request.form_data)
            .map_err(|err| ShieldError::Validation(err.to_string()))?;

        let result = self
            .client
            .user_management()
            .list_users(&ListUsersParams {
                pagination: PaginationParams {
                    limit: Some(1),
                    ..Default::default()
                },
                email: Some(&data.email),
                ..Default::default()
            })
            .await;

        info!("{result:?}");

        Ok(Response::Default)
    }
}

erased_action!(WorkosIndexAction);
