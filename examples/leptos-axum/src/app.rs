use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    components::{Outlet, ParentRoute, Router, Routes},
    path,
};
use shield_leptos::ShieldRouter;

// use crate::home::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/css/bootstrap.min.css"
                    integrity:="sha384-LN+7fdVzj6u52u30Kp6M/trliBMCMKTyK833zpbD+pXdCLuTusPj697FH4R/5mcr"
                    crossorigin="anonymous"
                />

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Shield Leptos Axum Example"/>

        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    // <Route path=path!("") view=HomePage />

                    <ParentRoute path=path!("auth") view=Outlet>
                        <ShieldRouter />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
