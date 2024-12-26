use std::sync::Arc;

use leptos::prelude::expect_context;
use shield::{ServerIntegration, Shield, User};

pub fn expect_server_integration<U: User + 'static>() -> Arc<dyn ServerIntegration<U>> {
    expect_context::<Arc<dyn ServerIntegration<U>>>()
}

pub async fn expect_shield<U: User + 'static>() -> Shield<U> {
    let server_integration = expect_server_integration();
    server_integration.extract_shield().await
}
