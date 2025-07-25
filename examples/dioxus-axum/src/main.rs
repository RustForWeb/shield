mod app;
mod home;

use crate::app::App;

#[cfg(not(feature = "server"))]
fn main() {
    use shield_bootstrap::BootstrapDioxusStyle;

    dioxus::LaunchBuilder::new()
        .with_context(BootstrapDioxusStyle::default().context())
        .launch(App)
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use dioxus::{
        cli_config::fullstack_address_or_localhost,
        prelude::{DioxusRouterExt, *},
    };
    use shield::{Shield, ShieldOptions};
    use shield_bootstrap::BootstrapDioxusStyle;
    use shield_dioxus_axum::{AxumDioxusIntegration, ShieldLayer};
    use shield_memory::{MemoryStorage, User};
    use shield_oidc::{Keycloak, OidcMethod};
    use tokio::net::TcpListener;
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};
    use tracing::{Level, info};

    // Initialize Dioxus
    let addr = fullstack_address_or_localhost();
    dioxus::logger::init(Level::DEBUG).unwrap();

    // Initialize sessions
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

    // Initialize Shield
    let storage = MemoryStorage::new();
    let shield = Shield::new(
        storage.clone(),
        vec![Arc::new(
            OidcMethod::new(storage).with_providers([Keycloak::builder(
                "keycloak",
                "http://localhost:18080/realms/Shield",
                "client1",
            )
            .client_secret("xcpQsaGbRILTljPtX4npjmYMBjKrariJ")
            .redirect_url(format!(
                "http://localhost:{}/api/auth/sign-in/callback/oidc/keycloak",
                addr.port()
            ))
            .build()]),
        )],
        ShieldOptions::default(),
    );
    let shield_layer = ShieldLayer::new(shield.clone());

    // Initialize router
    let router = Router::new()
        .serve_dioxus_application(
            ServeConfig::builder()
                .context(AxumDioxusIntegration::<User>::default().context())
                .context(BootstrapDioxusStyle::default().context())
                .build()
                .unwrap(),
            App,
        )
        .layer(shield_layer)
        .layer(session_layer);

    // Start app
    info!("listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
