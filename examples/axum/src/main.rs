use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::{Router, middleware::from_fn, routing::get};

    use shield::{Shield, ShieldOptions};
    use shield_axum::{AuthRoutes, ShieldLayer, auth_required};
    use shield_memory::{MemoryStorage, User};
    use shield_oidc::{Keycloak, OidcMethod};
    use time::Duration;
    use tokio::net::TcpListener;
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
    use tracing::{info, level_filters::LevelFilter};
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Configuration
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

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
                "http://localhost:{}/api/auth/oidc/sign-in-callback/keycloak",
                addr.port()
            ))
            .build()]),
        )],
        ShieldOptions::default(),
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
        .route("/api/protected", get(async || "Protected"))
        .route_layer(from_fn(auth_required::<User>))
        .nest("/api/auth", AuthRoutes::router::<User, ()>())
        .merge(SwaggerUi::new("/api-docs").url("/api/openapi.json", Docs::openapi()))
        .layer(shield_layer)
        .layer(session_layer);

    // Start app
    info!("listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
