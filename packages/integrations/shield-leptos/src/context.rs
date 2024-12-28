use std::sync::Arc;

use leptos::prelude::expect_context;
use shield::{ShieldDyn, User};

use crate::integration::LeptosIntegration;

pub fn expect_server_integration() -> Arc<dyn LeptosIntegration> {
    expect_context::<Arc<dyn LeptosIntegration>>()
}

pub async fn expect_shield() -> ShieldDyn {
    let server_integration = expect_server_integration();
    server_integration.extract_shield().await
}

pub async fn extract_user() -> Option<Arc<dyn User>> {
    let server_integration = expect_server_integration();
    server_integration.extract_user().await
}
