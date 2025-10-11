use std::any::Any;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    ErasedAction,
    action::Action,
    error::{SessionError, ShieldError},
    provider::Provider,
};

#[async_trait]
pub trait Method: Send + Sync {
    type Provider: Provider;
    type Session: DeserializeOwned + Serialize;

    fn id(&self) -> String;

    fn actions(&self) -> Vec<Box<dyn Action<Self::Provider, Self::Session>>>;

    fn action_by_id(
        &self,
        action_id: &str,
    ) -> Option<Box<dyn Action<Self::Provider, Self::Session>>> {
        self.actions()
            .into_iter()
            .find(|action| action.id() == action_id)
    }

    async fn providers(&self) -> Result<Vec<Self::Provider>, ShieldError>;

    async fn provider_by_id(
        &self,
        provider_id: Option<&str>,
    ) -> Result<Option<Self::Provider>, ShieldError> {
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

    async fn erased_providers(
        &self,
    ) -> Result<Vec<(Option<String>, Box<dyn Any + Send + Sync>)>, ShieldError>;

    async fn erased_provider_by_id(
        &self,
        provider_id: Option<&str>,
    ) -> Result<Option<Box<dyn Any + Send + Sync>>, ShieldError>;

    fn erased_deserialize_session(
        &self,
        value: Option<&str>,
    ) -> Result<Box<dyn Any + Send + Sync>, SessionError>;
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
            ) -> Result<Vec<(Option<String>, Box<dyn std::any::Any + Send + Sync>)>, $crate::ShieldError> {
                self.providers().await.map(|providers| {
                    providers
                        .into_iter()
                        .map(|provider| ($crate::Provider::id(&provider), Box::new(provider) as Box<dyn std::any::Any + Send + Sync>))
                        .collect()
                })
            }

            async fn erased_provider_by_id(
                &self,
                provider_id: Option<&str>,
            ) -> Result<Option<Box<dyn std::any::Any + Send + Sync>>, $crate::ShieldError> {
                self.provider_by_id(provider_id).await.map(|provider| {
                    provider
                        .map(|provider| Box::new(provider) as Box<dyn std::any::Any + Send + Sync>)
                })
            }

            fn erased_deserialize_session(
                &self,
                value: Option<&str>
            ) -> Result<Box<dyn std::any::Any + Send + Sync>, $crate::SessionError> {
                type Session $( < $( $generic_name ),+ > )* = <$method $( < $( $generic_name ),+ > )* as $crate::Method>::Session;

                let session = match value {
                    Some(value) => serde_json::from_str::<Session $( < $( $generic_name ),+ > )* >(value)
                        .map_err(|err| $crate::SessionError::Serialization(err.to_string()))?,
                    None => Session $( ::< $( $generic_name ),+ > )* ::default()
                };

                Ok(Box::new(session) as Box<dyn std::any::Any + Send + Sync>)
            }
        }
    };
}
