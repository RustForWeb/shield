use std::any::Any;

use async_trait::async_trait;

use crate::{
    error::ShieldError, form::Form, provider::Provider, request::Request, response::Response,
    session::Session,
};

pub const SIGN_IN_ACTION_ID: &str = "sign-in";
pub const SIGN_IN_CALLBACK_ACTION_ID: &str = "sign-in-callback";
pub const SIGN_OUT_ACTION_ID: &str = "sign-out";

#[async_trait]
pub trait Action<P: Provider>: ErasedAction + Send + Sync {
    fn id(&self) -> String;

    fn form(&self, provider: P) -> Form;

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

    fn erased_form(&self, provider: Box<dyn Any + Send + Sync>) -> Form;

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

            fn erased_form(&self, provider: Box<dyn std::any::Any + Send + Sync>) -> $crate::Form {
                self.form(*provider.downcast().expect("TODO"))
            }

            async fn erased_call(
                &self,
                provider: Box<dyn std::any::Any + Send + Sync>,
                session: $crate::Session,
                request: $crate::Request,
            ) -> Result<$crate::Response, ShieldError> {
                self.call(*provider.downcast().expect("TODO"), session, request)
                    .await
            }
        }
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::{
        error::ShieldError, form::Form, provider::tests::TestProvider, request::Request,
        response::Response, session::Session,
    };

    use super::Action;

    pub const TEST_ACTION_ID: &str = "action";

    #[derive(Default)]
    pub struct TestAction {}

    #[async_trait]
    impl Action<TestProvider> for TestAction {
        fn id(&self) -> String {
            TEST_ACTION_ID.to_owned()
        }

        fn form(&self, _provider: TestProvider) -> Form {
            Form { inputs: vec![] }
        }

        async fn call(
            &self,
            _provider: TestProvider,
            _session: Session,
            _request: Request,
        ) -> Result<Response, ShieldError> {
            Ok(Response::Default)
        }
    }

    erased_action!(TestAction);
}
