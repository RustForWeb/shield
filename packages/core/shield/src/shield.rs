use std::{collections::HashMap, sync::Arc};

use futures::future::try_join_all;

use crate::{
    MethodError,
    error::{SessionError, ShieldError},
    method::Method,
    options::ShieldOptions,
    provider::{Provider, ProviderVisualisation},
    request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
    response::Response,
    session::Session,
    storage::Storage,
    user::User,
};

#[derive(Clone)]
pub struct Shield<U: User> {
    storage: Arc<dyn Storage<U>>,
    methods: Arc<HashMap<String, Arc<dyn Method>>>,
    options: ShieldOptions,
}

impl<U: User> Shield<U> {
    pub fn new<S>(storage: S, providers: Vec<Arc<dyn Method>>, options: ShieldOptions) -> Self
    where
        S: Storage<U> + 'static,
    {
        Self {
            storage: Arc::new(storage),
            methods: Arc::new(
                providers
                    .into_iter()
                    .map(|provider| (provider.id(), provider))
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

    pub fn method_by_id(&self, provider_id: &str) -> Option<&dyn Method> {
        self.methods.get(provider_id).map(|v| &**v)
    }

    pub async fn providers(&self) -> Result<Vec<Box<dyn Provider>>, ShieldError> {
        try_join_all(self.methods.values().map(|provider| provider.providers()))
            .await
            .map(|providers| providers.into_iter().flatten().collect::<Vec<_>>())
    }

    pub async fn provider_visualisations(&self) -> Result<Vec<ProviderVisualisation>, ShieldError> {
        self.providers().await.map(|providers| {
            providers
                .into_iter()
                .map(|provider| {
                    let method_id = provider.method_id();
                    let provider_id = provider.id();

                    ProviderVisualisation {
                        key: match &provider_id {
                            Some(provider_id) => format!("{method_id}-{provider_id}"),
                            None => method_id.clone(),
                        },
                        method_id,
                        provider_id,
                        name: provider.name(),
                        icon_url: provider.icon_url(),
                    }
                })
                .collect()
        })
    }

    pub async fn provider_by_id(
        &self,
        method_id: &str,
        provider_id: Option<&str>,
    ) -> Result<Option<Box<dyn Provider>>, ShieldError> {
        match self.method_by_id(method_id) {
            Some(provider) => provider.provider_by_id(provider_id.expect("TODO")).await,
            None => Ok(None),
        }
    }

    pub async fn sign_in(
        &self,
        request: SignInRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        let provider = match self.methods.get(&request.method_id) {
            Some(provider) => provider,
            None => return Err(MethodError::MethodNotFound(request.method_id).into()),
        };

        // TODO: validate redirect URL

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.redirect_url = request.redirect_url.clone();
        };

        let response = provider
            .sign_in(request, session.clone(), &self.options)
            .await;

        session.update().await?;

        response
    }

    pub async fn sign_in_callback(
        &self,
        request: SignInCallbackRequest,
        session: Session,
    ) -> Result<Response, ShieldError> {
        let provider = match self.methods.get(&request.method_id) {
            Some(provider) => provider,
            None => return Err(MethodError::MethodNotFound(request.method_id).into()),
        };

        let redirect_url = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.redirect_url.clone()
        };

        let response = provider
            .sign_in_callback(
                SignInCallbackRequest {
                    redirect_url: request.redirect_url.or(redirect_url),
                    ..request
                },
                session.clone(),
                &self.options,
            )
            .await;

        session.update().await?;

        response
    }

    pub async fn sign_out(&self, session: Session) -> Result<Response, ShieldError> {
        let authenticated = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication.clone()
        };

        let response = if let Some(authenticated) = authenticated {
            let provider = match self.methods.get(&authenticated.method_id) {
                Some(provider) => provider,
                None => {
                    return Err(MethodError::MethodNotFound(authenticated.method_id).into());
                }
            };

            provider
                .sign_out(
                    SignOutRequest {
                        method_id: authenticated.method_id,
                        provider_id: authenticated.provider_id,
                    },
                    session.clone(),
                    &self.options,
                )
                .await?
        } else {
            None
        };

        let response =
            response.unwrap_or_else(|| Response::Redirect(self.options.sign_out_redirect.clone()));

        session.purge().await?;

        Ok(response)
    }

    pub async fn user(&self, session: &Session) -> Result<Option<U>, ShieldError> {
        let authentication = {
            let session_data = session.data();
            let session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication.clone()
        };

        match authentication {
            Some(authentication) => {
                if self
                    .provider_by_id(
                        &authentication.method_id,
                        authentication.provider_id.as_deref(),
                    )
                    .await?
                    .is_none()
                {
                    session.purge().await?;
                    return Ok(None);
                }

                let user = self.storage().user_by_id(&authentication.user_id).await?;

                if user.is_none() {
                    session.purge().await?;
                }

                Ok(user)
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        ShieldOptions,
        method::tests::{TEST_METHOD_ID, TestMethod},
        storage::tests::{TEST_STORAGE_ID, TestStorage},
    };

    use super::Shield;

    #[test]
    fn test_storage() {
        let shield = Shield::new(TestStorage::default(), vec![], ShieldOptions::default());

        assert_eq!(TEST_STORAGE_ID, shield.storage().id());
    }

    #[test]
    fn test_providers() {
        let shield = Shield::new(
            TestStorage::default(),
            vec![
                Arc::new(TestMethod::default().with_id("test1")),
                Arc::new(TestMethod::default().with_id("test2")),
            ],
            ShieldOptions::default(),
        );

        assert_eq!(
            None,
            shield
                .method_by_id(TEST_METHOD_ID)
                .map(|provider| provider.id())
        );
        assert_eq!(
            Some("test1".to_owned()),
            shield.method_by_id("test1").map(|provider| provider.id())
        );
        assert_eq!(
            Some("test2".to_owned()),
            shield.method_by_id("test2").map(|provider| provider.id())
        );
    }
}
