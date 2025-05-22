use leptos::{either::Either, prelude::*};
use shield::ProviderVisualisation;

#[server]
pub async fn providers() -> Result<Vec<ProviderVisualisation>, ServerFnError> {
    use crate::context::extract_shield;

    let shield = extract_shield().await;

    shield
        .provider_visualisations()
        .await
        .map_err(|err| err.into())
}

#[server]
pub async fn sign_in(method_id: String, provider_id: Option<String>) -> Result<(), ServerFnError> {
    use shield::{Response, ShieldError, SignInRequest};

    use crate::context::expect_server_integration;

    let server_integration = expect_server_integration();
    let shield = server_integration.extract_shield().await;
    let session = server_integration.extract_session().await;

    let response = shield
        .sign_in(
            SignInRequest {
                method_id,
                provider_id,
                redirect_url: None,
                data: None,
                form_data: None,
            },
            session,
        )
        .await
        .map_err(ServerFnError::<ShieldError>::from)?;

    match response {
        Response::Redirect(url) => {
            server_integration.redirect(&url);

            Ok(())
        }
    }
}

#[component]
pub fn SignIn() -> impl IntoView {
    let providers = OnceResource::new(providers());
    let sign_in = ServerAction::<SignIn>::new();

    view! {
        <h1>"Sign in"</h1>

        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match providers.await {
                Ok(providers) => Either::Left(view! {
                    <For
                        each=move || providers.clone()
                        key=move |provider| provider.key.clone()
                        let:provider
                    >
                        <ActionForm action=sign_in>
                            <input name="method_id" type="hidden" value=provider.method_id />
                            <input name="provider_id" type="hidden" value=provider.provider_id />

                            <button type="submit">{move || format!("Sign in with {}", &provider.name)}</button>
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
