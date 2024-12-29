#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;

    use actix_files::Files;
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::{cookie::Key, web::Data, App, HttpServer};
    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use shield::Shield;
    use shield_examples_leptos_actix::app::*;
    use shield_leptos_actix::{provide_actix_integration, ShieldMiddleware};
    use shield_memory::{MemoryStorage, User};
    use shield_oidc::{Keycloak, OidcProvider};
    use tracing::{info, level_filters::LevelFilter};

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Initialize Leptos
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate session key
    let session_secret_key = Key::generate();

    HttpServer::new(move || {
        // Initialize Leptos routes
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        info!("listening on http://{}", &addr);

        // Initialize sessions
        let session_middleware =
            SessionMiddleware::new(CookieSessionStore::default(), session_secret_key.clone());

        // Initialize Shield
        let shield_storage = MemoryStorage::new();
        let shield = Shield::new(
            shield_storage.clone(),
            vec![Arc::new(
                OidcProvider::new(shield_storage).with_subproviders([Keycloak::builder(
                    "keycloak",
                    "http://localhost:18080/realms/Shield",
                    "client1",
                )
                .client_secret("xcpQsaGbRILTljPtX4npjmYMBjKrariJ")
                .build()]),
            )],
        );
        let shield_middleware = ShieldMiddleware::new(shield.clone());

        // Initialize app
        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes_with_context(
                routes,
                move || {
                    provide_actix_integration::<User>();
                },
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                },
            )
            .app_data(Data::new(leptos_options.to_owned()))
            .wrap(shield_middleware)
            .wrap(session_middleware)
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
