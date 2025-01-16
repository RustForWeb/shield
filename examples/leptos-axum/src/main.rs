#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::config::{get_configuration, LeptosOptions};
    use leptos::logging::log;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shield::Shield;
    use shield_examples_leptos_axum::app::*;
    use shield_leptos_axum::{provide_axum_integration, AuthRoutes, ShieldLayer};
    use shield_memory::{MemoryStorage, User};
    use shield_oidc::{Keycloak, OidcProvider};
    use time::Duration;
    use tokio::net::TcpListener;
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
    use tracing::level_filters::LevelFilter;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Initialize Leptos
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

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
            OidcProvider::new(storage).with_subproviders([Keycloak::builder(
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
    );
    let shield_layer = ShieldLayer::new(shield.clone());

    // Initialize OpenAPI specification (optional)
    #[derive(OpenApi)]
    #[openapi(nest(
        (path = "/api/auth", api = AuthRoutes, tags = ["auth"]),
    ))]
    struct Docs;

    // Initialize router
    let router = Router::new()
        .nest("/api/auth", AuthRoutes::router::<User, LeptosOptions>())
        .merge(SwaggerUi::new("/api-docs").url("/api/openapi.json", Docs::openapi()))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_axum_integration::<User>();
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
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
