use leptos::prelude::*;

#[server]
pub async fn sign_out() -> Result<(), ServerFnError> {
    use shield::{Response, ShieldError};

    use crate::context::expect_server_integration;

    let server_integration = expect_server_integration();
    let shield = server_integration.extract_shield().await;
    let session = server_integration.extract_session().await;

    let response = shield
        .sign_out(session)
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
            <button type="submit">"Sign out"</button>
        </ActionForm>
    }
}
