use leptos::{either::Either, prelude::*};
use shield::SubproviderVisualisation;

#[server]
pub async fn subproviders() -> Result<Vec<SubproviderVisualisation>, ServerFnError> {
    #[cfg(feature = "actix")]
    {
        // use leptos_actix::extract;
        // TODO
    }
    #[cfg(feature = "axum")]
    {
        use leptos_axum::extract;
        use shield_axum::ExtractShield;

        let ExtractShield(shield) = extract::<ExtractShield>().await?;

        shield
            .subprovider_visualisations()
            .await
            .map_err(|err| err.into())
    }
}

#[server]
pub async fn sign_in(
    provider_id: String,
    subprovider_id: Option<String>,
) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use shield::SignInRequest;
    use shield_axum::ExtractShield;

    let ExtractShield(shield) = extract::<ExtractShield>().await?;

    shield
        .sign_in(SignInRequest {
            provider_id,
            subprovider_id,
            data: None,
            form_data: None,
        })
        .await
        .map_err(|err| err.into())
}

#[component]
pub fn SignIn() -> impl IntoView {
    let subproviders = OnceResource::new(subproviders());
    let sign_in = ServerAction::<SignIn>::new();

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
                        <ActionForm action=sign_in>
                            <input name="provider_id" type="hidden" value=subprovider.provider_id />
                            <input name="subprovider_id" type="hidden" value=subprovider.subprovider_id />

                            <button type="submit">{move || format!("Sign in with {}", &subprovider.name)}</button>
                        </ActionForm>
                    </For>
                }),
                Err(err) => Either::Right(view! {
                    {err.to_string()}
                })
            }})}
        </Suspense>
    }
}
