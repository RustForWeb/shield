use std::{any::Any, collections::HashMap, sync::Arc};

use futures::future::try_join_all;
use tracing::warn;

use crate::{
    ActionMethodForm,
    action::{ActionForms, ActionProviderForm},
    error::{ActionError, MethodError, ProviderError, ShieldError},
    method::ErasedMethod,
    options::ShieldOptions,
    request::Request,
    response::Response,
    session::Session,
    storage::Storage,
    user::User,
};

#[derive(Clone)]
pub struct Shield<U: User> {
    storage: Arc<dyn Storage<U>>,
    methods: Arc<HashMap<String, Arc<dyn ErasedMethod>>>,
    options: ShieldOptions,
}

impl<U: User> Shield<U> {
    pub fn new<S>(storage: S, methods: Vec<Arc<dyn ErasedMethod>>, options: ShieldOptions) -> Self
    where
        S: Storage<U> + 'static,
    {
        // TOOD: Check for duplicate method IDs.

        Self {
            storage: Arc::new(storage),
            methods: Arc::new(
                methods
                    .into_iter()
                    .map(|method| (method.erased_id(), method))
                    .collect(),
            ),
            options,
        }
    }

    pub fn storage(&self) -> &dyn Storage<U> {
        &*self.storage
    }

    pub fn options(&self) -> &ShieldOptions {
        &self.options
    }

    pub fn method_by_id(&self, method_id: &str) -> Option<&dyn ErasedMethod> {
        self.methods.get(method_id).map(|v| &**v)
    }

    pub async fn providers(&self) -> Result<Vec<Box<dyn Any + Send + Sync>>, ShieldError> {
        try_join_all(
            self.methods
                .values()
                .map(|provider| provider.erased_providers()),
        )
        .await
        .map(|providers| {
            providers
                .into_iter()
                .flat_map(|providers| providers.into_iter().map(|(_, provider)| provider))
                .collect::<Vec<_>>()
        })
    }

    pub async fn provider_by_id(
        &self,
        method_id: &str,
        provider_id: Option<&str>,
    ) -> Result<Option<Box<dyn Any + Send + Sync>>, ShieldError> {
        match self.method_by_id(method_id) {
            Some(provider) => provider.erased_provider_by_id(provider_id).await,
            None => Ok(None),
        }
    }

    pub async fn action_forms(
        &self,
        action_id: &str,
        session: Session,
    ) -> Result<ActionForms, ShieldError> {
        let mut action_name = None::<String>;
        let mut method_forms = vec![];

        for (method_id, method) in self.methods.iter() {
            let Some(action) = method.erased_action_by_id(action_id) else {
                continue;
            };

            let name = action.erased_name();
            if let Some(action_name) = &action_name
                && *action_name != name
            {
                warn!("Action name mismatch `{}` != `{}`", action_name, name);
            }
            action_name = Some(name);

            let mut provider_forms = vec![];
            for (provider_id, provider) in method.erased_providers().await? {
                if !action.erased_condition(&*provider, session.clone())? {
                    continue;
                }

                let forms = action.erased_forms(provider).await?;
                for form in forms {
                    provider_forms.push(ActionProviderForm {
                        id: provider_id.clone(),
                        form,
                    });
                }
            }

            method_forms.push(ActionMethodForm {
                id: method_id.clone(),
                provider_forms,
            });
        }

        Ok(ActionForms {
            id: action_id.to_owned(),
            name: action_name.unwrap_or(action_id.to_owned()),
            method_forms,
        })
    }

    pub async fn call(
        &self,
        action_id: &str,
        method_id: &str,
        provider_id: Option<&str>,
        session: Session,
        request: Request,
    ) -> Result<Response, ShieldError> {
        let method =
            self.method_by_id(method_id)
                .ok_or(ShieldError::Method(MethodError::NotFound(
                    method_id.to_owned(),
                )))?;

        let action = method
            .erased_action_by_id(action_id)
            .ok_or(ShieldError::Action(ActionError::NotFound(
                action_id.to_owned(),
            )))?;

        let provider =
            method
                .erased_provider_by_id(provider_id)
                .await?
                .ok_or(ShieldError::Provider(ProviderError::NotFound(
                    provider_id.map(ToOwned::to_owned),
                )))?;

        let response = action.erased_call(provider, session.clone(), request).await;

        // TODO: Should update always be called?
        session.update().await?;

        response
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        options::ShieldOptions,
        storage::tests::{TEST_STORAGE_ID, TestStorage},
    };

    use super::Shield;

    #[test]
    fn test_storage() {
        let shield = Shield::new(TestStorage::default(), vec![], ShieldOptions::default());

        assert_eq!(TEST_STORAGE_ID, shield.storage().id());
    }
}
