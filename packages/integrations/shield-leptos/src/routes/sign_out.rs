use leptos::prelude::*;

#[server]
pub async fn sign_out(
    provider_id: String,
    subprovider_id: Option<String>,
) -> Result<(), ServerFnError> {
    use shield::{Response, ShieldError, SignOutRequest};

    use crate::context::expect_server_integration;

    let server_integration = expect_server_integration();
    let shield = server_integration.extract_shield().await;
    let session = server_integration.extract_session().await;

    let response = shield
        .sign_out(
            SignOutRequest {
                provider_id,
                subprovider_id,
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
pub fn SignOut() -> impl IntoView {
    let sign_out = ServerAction::<SignOut>::new();

    view! {
        <h1>"Sign out"</h1>

        <ActionForm action=sign_out>
            // <input name="provider_id" type="hidden" value=subprovider.provider_id />
            // <input name="subprovider_id" type="hidden" value=subprovider.subprovider_id />

            <button type="submit">"Sign out"</button>
        </ActionForm>
    }
}
