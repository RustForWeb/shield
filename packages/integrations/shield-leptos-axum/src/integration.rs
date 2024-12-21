use std::sync::Arc;

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_axum::{extract, redirect};
use shield::{ServerIntegration, Session, Shield};
use shield_axum::{ExtractSession, ExtractShield};

pub struct LeptosAxumIntegration;

#[async_trait]
impl ServerIntegration for LeptosAxumIntegration {
    async fn extract_shield(&self) -> Shield {
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

pub fn provide_axum_integration() {
    provide_context::<Arc<dyn ServerIntegration>>(Arc::new(LeptosAxumIntegration));
}
