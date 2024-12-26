use std::sync::Arc;

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_actix::{extract, redirect};
use shield::{ServerIntegration, Session, Shield, User};
use shield_actix::{ExtractSession, ExtractShield};

pub struct LeptosActixIntegration;

#[async_trait]
impl<U: User + Clone + 'static> ServerIntegration<U> for LeptosActixIntegration {
    async fn extract_shield(&self) -> Shield<U> {
        let ExtractShield(shield) = extract().await.expect("TOD");
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

pub fn provide_actix_integration<U: User + Clone + 'static>() {
    provide_context::<Arc<dyn ServerIntegration<U>>>(Arc::new(LeptosActixIntegration));
}
