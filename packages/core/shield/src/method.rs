use std::any::Any;

use async_trait::async_trait;

use crate::{ErasedAction, action::Action, error::ShieldError, provider::Provider};

#[async_trait]
pub trait Method<P: Provider>: Send + Sync {
    fn id(&self) -> String;

    fn actions(&self) -> Vec<Box<dyn Action<P>>>;

    fn action_by_id(&self, action_id: &str) -> Option<Box<dyn Action<P>>> {
        self.actions()
            .into_iter()
            .find(|action| action.id() == action_id)
    }

    async fn providers(&self) -> Result<Vec<P>, ShieldError>;

    async fn provider_by_id(&self, provider_id: Option<&str>) -> Result<Option<P>, ShieldError> {
        Ok(self
            .providers()
            .await?
            .into_iter()
            .find(|provider| provider.id().as_deref() == provider_id))
    }
}

#[async_trait]
pub trait ErasedMethod: Send + Sync {
    fn erased_id(&self) -> String;

    fn erased_actions(&self) -> Vec<Box<dyn ErasedAction>>;

    fn erased_action_by_id(&self, action_id: &str) -> Option<Box<dyn ErasedAction>>;

    async fn erased_providers(&self) -> Result<Vec<Box<dyn Any + Send + Sync>>, ShieldError>;

    async fn erased_provider_by_id(
        &self,
        provider_id: Option<&str>,
    ) -> Result<Option<Box<dyn Any + Send + Sync>>, ShieldError>;
}

#[macro_export]
macro_rules! erased_method {
    ($method:ident $(, < $( $generic_name:ident : $generic_type:ident ),+ > )*) => {
        #[async_trait]
        impl $( < $( $generic_name: $generic_type + 'static ),+ > )* $crate::ErasedMethod for $method $( < $( $generic_name ),+ > )* {
            fn erased_id(&self) -> String {
                self.id()
            }

            fn erased_actions(&self) -> Vec<Box<dyn $crate::ErasedAction>> {
                self.actions()
                    .into_iter()
                    .map(|action| action as Box<dyn $crate::ErasedAction>)
                    .collect()
            }

            fn erased_action_by_id(
                &self,
                action_id: &str,
            ) -> Option<Box<dyn $crate::ErasedAction>> {
                self.action_by_id(action_id)
                    .map(|action| action as Box<dyn $crate::ErasedAction>)
            }

            async fn erased_providers(
                &self,
            ) -> Result<Vec<Box<dyn std::any::Any + Send + Sync>>, ShieldError> {
                self.providers().await.map(|providers| {
                    providers
                        .into_iter()
                        .map(|provider| Box::new(provider) as Box<dyn std::any::Any + Send + Sync>)
                        .collect()
                })
            }

            async fn erased_provider_by_id(
                &self,
                provider_id: Option<&str>,
            ) -> Result<Option<Box<dyn std::any::Any + Send + Sync>>, ShieldError> {
                self.provider_by_id(provider_id).await.map(|provider| {
                    provider
                        .map(|provider| Box::new(provider) as Box<dyn std::any::Any + Send + Sync>)
                })
            }
        }
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::{
        action::{Action, tests::TestAction},
        error::ShieldError,
        provider::tests::TestProvider,
    };

    use super::Method;

    pub const TEST_METHOD_ID: &str = "test";

    #[derive(Default)]
    pub struct TestMethod {
        id: Option<&'static str>,
    }

    impl TestMethod {
        // TODO
        pub fn _with_id(mut self, id: &'static str) -> Self {
            self.id = Some(id);
            self
        }
    }

    #[async_trait]
    impl Method<TestProvider> for TestMethod {
        fn id(&self) -> String {
            self.id.unwrap_or(TEST_METHOD_ID).to_owned()
        }

        fn actions(&self) -> Vec<Box<dyn Action<TestProvider>>> {
            vec![Box::new(TestAction::default())]
        }

        async fn providers(&self) -> Result<Vec<TestProvider>, ShieldError> {
            Ok(vec![TestProvider::default()])
        }
    }

    erased_method!(TestMethod);
}
