use std::any::Any;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::ShieldError, form::Form, provider::Provider, request::Request, response::Response,
    session::Session,
};

// TODO: Think of a better name.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionForms {
    pub id: String,
    pub name: String,
    pub method_forms: Vec<ActionMethodForm>,
}

// TODO: Think of a better name.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActionMethodForm {
    pub id: String,
    pub provider_forms: Vec<ActionProviderForm>,
}

// TODO: Think of a better name.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActionProviderForm {
    pub id: Option<String>,
    pub form: Form,
}

#[async_trait]
pub trait Action<P: Provider>: ErasedAction + Send + Sync {
    fn id(&self) -> String;

    fn name(&self) -> String;

    fn condition(&self, _provider: &P, _session: Session) -> Result<bool, ShieldError> {
        Ok(true)
    }

    async fn forms(&self, provider: P) -> Result<Vec<Form>, ShieldError>;

    async fn call(
        &self,
        provider: P,
        session: Session,
        request: Request,
    ) -> Result<Response, ShieldError>;
}

#[async_trait]
pub trait ErasedAction: Send + Sync {
    fn erased_id(&self) -> String;

    fn erased_name(&self) -> String;

    fn erased_condition(
        &self,
        provider: &(dyn Any + Send + Sync),
        session: Session,
    ) -> Result<bool, ShieldError>;

    async fn erased_forms(
        &self,
        provider: Box<dyn Any + Send + Sync>,
    ) -> Result<Vec<Form>, ShieldError>;

    async fn erased_call(
        &self,
        provider: Box<dyn Any + Send + Sync>,
        session: Session,
        request: Request,
    ) -> Result<Response, ShieldError>;
}

#[macro_export]
macro_rules! erased_action {
    ($action:ident $(, < $( $generic_name:ident : $generic_type:ident ),+ > )*) => {
        #[async_trait]
        impl $( < $( $generic_name: $generic_type + 'static ),+ > )* $crate::ErasedAction for $action $( < $( $generic_name ),+ > )* {
            fn erased_id(&self) -> String {
                self.id()
            }

            fn erased_name(&self) -> String {
                self.name()
            }

            fn erased_condition(&self, provider: &(dyn std::any::Any + Send + Sync), session: $crate::Session) -> Result<bool, $crate::ShieldError> {
                self.condition(provider.downcast_ref().expect("TODO"), session)
            }

            async fn erased_forms(&self, provider: Box<dyn std::any::Any + Send + Sync>) -> Result<Vec<$crate::Form>, $crate::ShieldError> {
                self.forms(*provider.downcast().expect("TODO")).await
            }

            async fn erased_call(
                &self,
                provider: Box<dyn std::any::Any + Send + Sync>,
                session: $crate::Session,
                request: $crate::Request,
            ) -> Result<$crate::Response, $crate::ShieldError> {
                self.call(*provider.downcast().expect("TODO"), session, request)
                    .await
            }
        }
    };
}
