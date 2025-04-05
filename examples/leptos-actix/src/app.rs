use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use shield_leptos::routes::{SignIn, SignOut};

use crate::home::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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
        <Title text="Shield Leptos Actix Example"/>

        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("") view=HomePage />

                    <Route path=path!("/auth/sign-in") view=SignIn />
                    <Route path=path!("/auth/sign-out") view=SignOut />
                </Routes>
            </main>
        </Router>
    }
}
