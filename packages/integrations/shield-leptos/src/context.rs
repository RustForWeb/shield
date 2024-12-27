use std::sync::Arc;

use leptos::prelude::expect_context;
use shield::{Shield, User};

use crate::integration::LeptosIntegration;

pub fn expect_server_integration<U: User + 'static>() -> Arc<dyn LeptosIntegration<U>> {
    expect_context::<Arc<dyn LeptosIntegration<U>>>()
}

pub async fn expect_shield<U: User + 'static>() -> Shield<U> {
    let server_integration = expect_server_integration();
    server_integration.extract_shield().await
}
