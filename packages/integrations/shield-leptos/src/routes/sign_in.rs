use leptos::{either::Either, prelude::*};
use shield::SubproviderVisualisation;

#[server]
pub async fn subproviders() -> Result<Vec<SubproviderVisualisation>, ServerFnError> {
    use shield_axum::ExtractShield;

    use crate::extract::extract;

    let ExtractShield(shield) = extract::<ExtractShield>().await?;

    shield
        .subprovider_visualisations()
        .await
        .map_err(|err| err.into())
}

#[component]
pub fn SignIn() -> impl IntoView {
    let subproviders = OnceResource::new(subproviders());

    view! {
        <h1>"Sign in"</h1>

        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match subproviders.await {
                Ok(subproviders) => Either::Left(view! {
                    <For
                        each=move || subproviders.clone()
                        key=move |subprovider| subprovider.key.clone()
                        let:subprovider
                    >
                        // TODO: Leptos action form?
                        <form method="post" action="/api/auth/sign-in">
                            <input name="provider_id" type="hidden" value=subprovider.provider_id />
                            <input name="subprovider_id" type="hidden" value=subprovider.subprovider_id />

                            <button type="submit">{move || format!("Sign in with {}", &subprovider.name)}</button>
                        </form>
                    </For>
                }),
                Err(err) => Either::Right(view! {
                    {err.to_string()}
                })
            }})}
        </Suspense>
    }
}
