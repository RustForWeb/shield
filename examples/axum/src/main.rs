use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::{Json, middleware::from_fn, routing::get};
use shield::{Shield, ShieldOptions};
use shield_axum::{AuthRoutes, ShieldLayer, auth_required};
use shield_memory::{MemoryStorage, User};
use shield_oidc::{Keycloak, OidcMethod};
use time::Duration;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::{info, level_filters::LevelFilter};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Configuration
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

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

    // Initialize API router
    let api_router = OpenApiRouter::new()
        .route("/protected", get(async || "Protected"))
        .route_layer(from_fn(auth_required::<User>))
        .nest("/auth", AuthRoutes::openapi_router::<User, ()>());

    // Initialize router
    let (router, openapi) = OpenApiRouter::new()
        .nest("/api", api_router)
        .layer(shield_layer)
        .layer(session_layer)
        .split_for_parts();

    // Add Scalar and OpenAPI specification
    let router = router
        .merge(Scalar::with_url("/api/reference", openapi.clone()))
        .route("/api/openapi.json", get(|| async { Json(openapi) }));

    // Start app
    info!("listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
