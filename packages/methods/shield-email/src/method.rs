use std::sync::Arc;

use async_trait::async_trait;
use shield::{Action, Method, ShieldError, User, erased_method};

use crate::{
    actions::{EmailSignInAction, EmailSignInCallbackAction, EmailSignOutAction},
    options::EmailOptions,
    provider::EmailProvider,
    storage::EmailStorage,
};

pub const EMAIL_METHOD_ID: &str = "email";

pub struct EmailMethod<U: User> {
    options: EmailOptions,
    storage: Arc<dyn EmailStorage<U>>,
}

impl<U: User> EmailMethod<U> {
    pub fn new<S: EmailStorage<U> + 'static>(options: EmailOptions, storage: S) -> Self {
        Self {
            options,
            storage: Arc::new(storage),
        }
    }
}

#[async_trait]
impl<U: User + 'static> Method for EmailMethod<U> {
    type Provider = EmailProvider;
    type Session = ();

    fn id(&self) -> String {
        EMAIL_METHOD_ID.to_owned()
    }

    fn actions(&self) -> Vec<Box<dyn Action<Self::Provider, Self::Session>>> {
        vec![
            Box::new(EmailSignInAction::new(
                self.options.clone(),
                self.storage.clone(),
            )),
            Box::new(EmailSignInCallbackAction::new(
                self.options.clone(),
                self.storage.clone(),
            )),
            Box::new(EmailSignOutAction),
        ]
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError> {
        Ok(vec![EmailProvider])
    }
}

erased_method!(EmailMethod, <U: User>);
