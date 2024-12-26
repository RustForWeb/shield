use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};
use shield_leptos::routes::SignIn;

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
        <Title text="Shield Leptos Axum Example"/>

        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("") view=HomePage/>

                    <Route path=path!("/auth/sign-in") view=SignIn />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Shield Leptos Axum Example"</h1>
        <A href="/auth/sign-in">
            <button>"Sign in"</button>
        </A>
    }
}
