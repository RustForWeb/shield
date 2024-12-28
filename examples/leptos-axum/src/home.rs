use std::sync::Arc;

use leptos::prelude::*;
use tracing::debug;

#[server]
pub async fn user() -> Result<Option<Arc<dyn shield::User>>, ServerFnError> {
    use shield_leptos::context::extract_user;

    let user = extract_user().await;
    debug!("action {:?}", user.as_ref().map(|user| user.id()));

    Ok(user)
}

#[component]
pub fn HomePage() -> impl IntoView {
    let user = OnceResource::new(user());

    view! {
        <h1>"Shield Leptos Axum Example"</h1>

        {move || match user.get() {
            Some(user) => {
                debug!("{:?}", user);
                match user {
                    Ok(user) => view! {
                        {user.as_ref().map(|user| user.id())}
                    }.into_any(),
                    Err(err) => view! {
                        {err.to_string()}
                    }.into_any(),
                }
            }.into_any(),
            None => view! { "Loading..." }.into_any(),
        }}

        // <Suspense fallback=move || view! { "Loading..." }>
        //     {move || Suspend::new(async move {
        //         let user = user.await;
        //         debug!("view {:?}", user);

        //         match user {
        //             Ok(user) => Either::Left(view! {
        //                 {user.as_ref().map(|user| user.id())}
        //             }),
        //             Err(err) => Either::Right(view! {
        //                 {err.to_string()}
        //             })
        //         }
        //     })}
        // </Suspense>
    }
}
