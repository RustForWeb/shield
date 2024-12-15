#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::{config::get_configuration, prelude::provide_context};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shield::{DummyProvider, DummyStorage, Shield};
    use shield_axum::ShieldLayer;
    use shield_examples_leptos_axum::app::*;
    use shield_oidc::{KeycloakBuilder, OidcProvider};
    use time::Duration;
    use tokio::net::TcpListener;
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

    // Initialize Leptos
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // Initialize sessions
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    // Initialize Shield
    let shield = Shield::new(
        DummyStorage::new(),
        vec![
            Arc::new(DummyProvider::new()),
            Arc::new(
                OidcProvider::new().with_subproviders([KeycloakBuilder::new(
                    "keycloak",
                    "http://localhost:18080/realms/Shield",
                    "client1",
                )
                .client_secret("xcpQsaGbRILTljPtX4npjmYMBjKrariJ")
                .build()]),
            ),
        ],
    );
    let shield_layer = ShieldLayer::new(shield.clone());

    // Initialize app
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(shield.clone());
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(shield_layer)
        .layer(session_layer);

    // Start app
    log!("listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
