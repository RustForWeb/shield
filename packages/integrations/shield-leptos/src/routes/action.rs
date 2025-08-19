use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use shield::{ActionForms, Response};

use crate::ErasedLeptosStyle;

#[derive(Params, PartialEq)]
struct ActionParams {
    action_id: Option<String>,
}

#[component]
pub fn Action() -> impl IntoView {
    let params = use_params::<ActionParams>();
    let action_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.action_id.clone())
            .expect("TODO: Properly handle missing param.")
    };

    let resource = Resource::new(action_id, forms);
    let style = expect_context::<ErasedLeptosStyle>();

    view! {
        <Transition>
            {move || resource.get().map(|response| match response {
                Ok(forms) => style.render(&forms),
                Err(err) => format!("{err:?}").into_any()
            })}
        </Transition>
    }
}

#[server]
async fn forms(action_id: String) -> Result<ActionForms, ServerFnError> {
    use crate::expect_server_integration;

    let integration = expect_server_integration();
    let shield = integration.extract_shield().await;
    let session = integration.extract_session().await;

    let forms = shield.action_forms(&action_id, session).await?;

    Ok(forms)
}

#[server]
pub async fn call(
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
) -> Result<(), ServerFnError> {
    use serde_json::Value;
    use shield::Request;

    use crate::expect_server_integration;

    let integration = expect_server_integration();
    let shield = integration.extract_shield().await;
    let session = integration.extract_session().await;

    let response = shield
        .call(
            &action_id,
            &method_id,
            provider_id.as_deref(),
            session,
            // TODO: Support request input.
            Request {
                query: Value::Null,
                form_data: Value::Null,
            },
        )
        .await?;

    match response {
        Response::Default => todo!("default reponse"),
        Response::Redirect(to) => {
            integration.redirect(&to);
        }
    }

    Ok(())
}
