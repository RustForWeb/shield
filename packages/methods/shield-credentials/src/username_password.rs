use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use serde::Deserialize;
use shield::{Form, Input, InputType, InputTypePassword, InputTypeText, ShieldError, User};

use crate::Credentials;

#[derive(Debug, Deserialize)]
pub struct UsernamePasswordData {
    pub username: String,
    pub password: String,
}

type SignInFn<U> = dyn Fn(UsernamePasswordData) -> Pin<Box<dyn Future<Output = Result<U, ShieldError>> + Send + Sync>>
    + Send
    + Sync;

pub struct UsernamePasswordCredentials<U: User> {
    sign_in_fn: Arc<SignInFn<U>>,
}

impl<U: User> UsernamePasswordCredentials<U> {
    pub fn new(
        sign_in_fn: impl Fn(
            UsernamePasswordData,
        )
            -> Pin<Box<dyn Future<Output = Result<U, ShieldError>> + Send + Sync>>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            sign_in_fn: Arc::new(sign_in_fn),
        }
    }
}

#[async_trait]
impl<U: User> Credentials<U, UsernamePasswordData> for UsernamePasswordCredentials<U> {
    fn form(&self) -> Form {
        Form {
            inputs: vec![
                Input {
                    name: "username".to_owned(),
                    label: Some("Username".to_owned()),
                    r#type: InputType::Text(InputTypeText {
                        autocomplete: Some("username".to_owned()),
                        placeholder: Some("Username".to_owned()),
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
            ],
        }
    }

    async fn sign_in(&self, data: UsernamePasswordData) -> Result<U, ShieldError> {
        (self.sign_in_fn)(data).await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use shield::{EmailAddress, ShieldError, StorageError, User};

    use crate::Credentials;

    use super::{UsernamePasswordCredentials, UsernamePasswordData};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TestUser {
        id: String,
        name: Option<String>,
    }

    #[async_trait]
    impl User for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }

        fn name(&self) -> Option<String> {
            self.name.clone()
        }

        async fn email_addresses(&self) -> Result<Vec<EmailAddress>, StorageError> {
            Ok(vec![])
        }

        fn additional(&self) -> Option<impl Serialize> {
            None::<()>
        }
    }

    #[tokio::test]
    async fn username_password_credentials() -> Result<(), ShieldError> {
        let credentials = UsernamePasswordCredentials::new(|data: UsernamePasswordData| {
            Box::pin(async move {
                if data.username == "test" && data.password == "test" {
                    Ok(TestUser {
                        id: "1".to_owned(),
                        name: Some("Test".to_owned()),
                    })
                } else {
                    Err(ShieldError::Validation(
                        "Incorrect username and password combination.".to_owned(),
                    ))
                }
            })
        });

        assert!(
            credentials
                .sign_in(UsernamePasswordData {
                    username: "test".to_owned(),
                    password: "incorrect".to_owned(),
                })
                .await
                .is_err_and(|err| err
                    .to_string()
                    .contains("Incorrect username and password combination."))
        );

        let user = credentials
            .sign_in(UsernamePasswordData {
                username: "test".to_owned(),
                password: "test".to_owned(),
            })
            .await?;

        assert_eq!(user.name, Some("Test".to_owned()));

        Ok(())
    }
}
