use std::sync::Arc;

use leptos::prelude::expect_context;
use shield::{ServerIntegration, Shield};

pub fn expect_server_integration() -> Arc<dyn ServerIntegration> {
    expect_context::<Arc<dyn ServerIntegration>>()
}

pub async fn expect_shield() -> Shield {
    let server_integration = expect_server_integration();
    server_integration.extract_shield().await
}
