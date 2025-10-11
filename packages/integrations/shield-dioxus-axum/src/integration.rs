use std::marker::PhantomData;

use async_trait::async_trait;
use dioxus_server::extract;

use shield::{Session, ShieldDyn, User};
use shield_axum::{ExtractSession, ExtractShield};
use shield_dioxus::{DioxusIntegration, DioxusIntegrationDyn};

pub struct AxumDioxusIntegration<U: User>(PhantomData<U>);

impl<U: User + Clone + 'static> AxumDioxusIntegration<U> {
    pub fn context(self) -> DioxusIntegrationDyn {
        DioxusIntegrationDyn::new(self)
    }
}

impl<U: User> Default for AxumDioxusIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<U: User + Clone + 'static> DioxusIntegration for AxumDioxusIntegration<U> {
    async fn extract_shield(&self) -> ShieldDyn {
        let ExtractShield(shield) = extract::<ExtractShield<U>, _>()
            .await
            .expect("Shield should be extracted");

        ShieldDyn::new(shield)
    }

    async fn extract_session(&self) -> Session {
        let ExtractSession(session) = extract().await.expect("Session should be extracted");

        session
    }
}
