use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_axum::{extract, redirect};
use shield::{Session, ShieldDyn, User};
use shield_axum::{ExtractSession, ExtractShield};
use shield_leptos::LeptosIntegration;

pub struct LeptosAxumIntegration<U: User>(PhantomData<U>);

impl<U: User> Default for LeptosAxumIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<U: User + Clone + 'static> LeptosIntegration for LeptosAxumIntegration<U> {
    async fn extract_shield(&self) -> ShieldDyn {
        let ExtractShield(shield) = extract::<ExtractShield<U>>().await.expect("TODO");

        ShieldDyn::new(shield)
    }

    async fn extract_session(&self) -> Session {
        let ExtractSession(session) = extract().await.expect("TODO");
        session
    }

    fn redirect(&self, path: &str) {
        redirect(path);
    }
}

pub fn provide_axum_integration<U: User + Clone + 'static>() {
    provide_context::<Arc<dyn LeptosIntegration>>(Arc::new(LeptosAxumIntegration::<U>::default()));
}
