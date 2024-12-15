#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;

    use actix_files::Files;
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::{cookie::Key, web::Data, App, HttpServer};
    use leptos::{config::get_configuration, prelude::provide_context};
    use leptos_actix::{generate_route_list, redirect, LeptosRoutes};
    use shield::{DummyProvider, DummyStorage, Shield};
    use shield_actix::ShieldMiddleware;
    use shield_examples_leptos_actix::app::*;
    use shield_leptos::context::LeptosRedirect;
    use shield_oidc::{KeycloakBuilder, OidcProvider};

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

        println!("listening on http://{}", &addr);

        // Initialize sessions
        let session_middleware =
            SessionMiddleware::new(CookieSessionStore::default(), session_secret_key.clone());

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
        let shield_middleware = ShieldMiddleware::new(shield.clone());

        // Initialize app
        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes_with_context(
                routes,
                move || {
                    provide_context(shield.clone());
                    provide_context(LeptosRedirect::from(redirect));
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
