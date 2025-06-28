use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use serde::Deserialize;
use shield::{Form, Input, InputType, InputTypeEmail, InputTypePassword, ShieldError, User};

use crate::Credentials;

#[derive(Debug, Deserialize)]
pub struct EmailPasswordData {
    pub email: String,
    pub password: String,
}

type SignInFn<U> = dyn Fn(EmailPasswordData) -> Pin<Box<dyn Future<Output = Result<U, ShieldError>> + Send + Sync>>
    + Send
    + Sync;

pub struct EmailPasswordCredentials<U: User> {
    sign_in_fn: Arc<SignInFn<U>>,
}

impl<U: User> EmailPasswordCredentials<U> {
    pub fn new(
        sign_in_fn: impl Fn(
            EmailPasswordData,
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
impl<U: User> Credentials<U, EmailPasswordData> for EmailPasswordCredentials<U> {
    fn form(&self) -> Form {
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
                    attributes: None,
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
                    attributes: None,
                },
            ],
            attributes: None,
        }
    }

    async fn sign_in(&self, data: EmailPasswordData) -> Result<U, ShieldError> {
        (self.sign_in_fn)(data).await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use shield::{EmailAddress, ShieldError, StorageError, User};

    use crate::Credentials;

    use super::{EmailPasswordCredentials, EmailPasswordData};

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
    async fn email_password_credentials() -> Result<(), ShieldError> {
        let credentials = EmailPasswordCredentials::new(|data: EmailPasswordData| {
            Box::pin(async move {
                if data.email == "test@example.com" && data.password == "test" {
                    Ok(TestUser {
                        id: "1".to_owned(),
                        name: Some("Test".to_owned()),
                    })
                } else {
                    Err(ShieldError::Validation(
                        "Incorrect email and password combination.".to_owned(),
                    ))
                }
            })
        });

        assert!(
            credentials
                .sign_in(EmailPasswordData {
                    email: "test@example.com".to_owned(),
                    password: "incorrect".to_owned(),
                })
                .await
                .is_err_and(|err| err
                    .to_string()
                    .contains("Incorrect email and password combination."))
        );

        let user = credentials
            .sign_in(EmailPasswordData {
                email: "test@example.com".to_owned(),
                password: "test".to_owned(),
            })
            .await?;

        assert_eq!(user.name, Some("Test".to_owned()));

        Ok(())
    }
}
