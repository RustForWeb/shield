use std::sync::Arc;

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_axum::{extract, redirect};
use shield::{Session, Shield, User};
use shield_axum::{ExtractSession, ExtractShield};
use shield_leptos::LeptosIntegration;

pub struct LeptosAxumIntegration;

#[async_trait]
impl<U: User + Clone + 'static> LeptosIntegration<U> for LeptosAxumIntegration {
    async fn extract_shield(&self) -> Shield<U> {
        let ExtractShield(shield) = extract().await.expect("TODO");
        shield
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
    provide_context::<Arc<dyn LeptosIntegration<U>>>(Arc::new(LeptosAxumIntegration));
}
