use std::sync::Arc;

use leptos::{either::Either, prelude::*};
use leptos_router::components::A;

#[server]
pub async fn user() -> Result<Option<Arc<dyn shield::User>>, ServerFnError> {
    use shield_leptos::context::extract_user;

    Ok(extract_user().await)
}

#[component]
pub fn HomePage() -> impl IntoView {
    let user = OnceResource::new(user());

    view! {
        <h1>"Shield Leptos Actix Example"</h1>

        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match user.await {
                Ok(user) => Either::Left(match user {
                    Some(_user) => Either::Left(view! {
                        <A href="/auth/sign-out">
                            <button>"Sign out"</button>
                        </A>
                    }),
                    None => Either::Right(view! {
                        <A href="/auth/sign-in">
                            <button>"Sign in"</button>
                        </A>
                    }),
                }),
                Err(err) => Either::Right(view! {
                    {err.to_string()}
                })
            }})}
        </Suspense>
    }
}
