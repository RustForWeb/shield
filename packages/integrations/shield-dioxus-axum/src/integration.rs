use std::marker::PhantomData;

use async_trait::async_trait;
use dioxus::prelude::extract;

use shield::{Session, ShieldDyn, User};
use shield_axum::{ExtractSession, ExtractShield};
use shield_dioxus::{DioxusIntegration, DioxusIntegrationDyn};

pub struct DioxusAxumIntegration<U: User>(PhantomData<U>);

impl<U: User> Default for DioxusAxumIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<U: User + Clone + 'static> DioxusIntegration for DioxusAxumIntegration<U> {
    async fn extract_shield(&self) -> ShieldDyn {
        let ExtractShield(shield) = extract::<ExtractShield<U>, _>().await.expect("TODO");

        ShieldDyn::new(shield)
    }

    async fn extract_session(&self) -> Session {
        let ExtractSession(session) = extract().await.expect("TODO");

        session
    }
}

pub fn provide_axum_integration<U: User + Clone + 'static>() -> DioxusIntegrationDyn {
    DioxusIntegrationDyn::new(DioxusAxumIntegration::<U>::default())
}
