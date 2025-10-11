use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_axum::{extract, redirect};
use shield::{Session, ShieldDyn, User};
use shield_axum::{ExtractSession, ExtractShield, ExtractUser};
use shield_leptos::{LeptosIntegration, LeptosUser};

pub struct AxumLeptosIntegration<U: User>(PhantomData<U>);

impl<U: User> Default for AxumLeptosIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<U: User + Clone + 'static> LeptosIntegration for AxumLeptosIntegration<U> {
    async fn extract_shield(&self) -> ShieldDyn {
        let ExtractShield(shield) = extract::<ExtractShield<U>>()
            .await
            .expect("Shield should be extracted");

        ShieldDyn::new(shield)
    }

    async fn extract_session(&self) -> Session {
        let ExtractSession(session) = extract().await.expect("Session should be extracted");

        session
    }

    async fn extract_user(&self) -> Option<LeptosUser> {
        let ExtractUser(user) = extract::<ExtractUser<U>>()
            .await
            .expect("User should be extracted");

        user.map(|user| user.into())
    }

    fn redirect(&self, path: &str) {
        redirect(path);
    }
}

pub fn provide_axum_integration<U: User + Clone + 'static>() {
    provide_context::<Arc<dyn LeptosIntegration>>(Arc::new(AxumLeptosIntegration::<U>::default()));
}
