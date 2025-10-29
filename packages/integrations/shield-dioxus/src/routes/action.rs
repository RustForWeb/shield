use dioxus::prelude::*;
use serde_json::Value;
use shield::{ActionForms, ResponseType};

use crate::ErasedDioxusStyle;

#[derive(Clone, PartialEq, Props)]
pub struct ActionProps {
    #[props(default = "index".to_owned())]
    action_id: String,
}

#[component]
pub fn Action(props: ActionProps) -> Element {
    let response = use_server_future({
        let action_id = props.action_id.clone();

        move || forms(action_id.clone())
    })?;
    let style = use_context::<ErasedDioxusStyle>();

    let response_read = response.read();
    let response = response_read.as_ref().unwrap();

    match response {
        Ok(forms) => style.render(forms),
        Err(err) => rsx! { "{err}" },
    }
}

#[get("/api/auth/forms", parts: dioxus::fullstack::http::request::Parts)]
async fn forms(action_id: String) -> Result<ActionForms> {
    use anyhow::anyhow;

    use crate::integration::DioxusIntegrationDyn;

    let integration = parts
        .extensions
        .get::<DioxusIntegrationDyn>()
        .ok_or_else(|| anyhow!("Dioxus Shield integration should be extracted."))?;
    let shield = integration.extract_shield(&parts.extensions)?;
    let session = integration.extract_session(&parts.extensions)?;

    let forms = shield
        .action_forms(&action_id, session)
        .await
        .context("Failed to get Shield action forms.")?;

    Ok(forms)
}

#[post("/api/auth/call", parts: dioxus::fullstack::http::request::Parts)]
pub async fn call(
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
    // TODO: Would be nice if this argument could fill up with all unknown keys instead of setting name to `data[...]`.
    data: Value,
) -> Result<ResponseType> {
    use anyhow::anyhow;
    use serde_json::Value;
    use shield::Request;

    use crate::integration::DioxusIntegrationDyn;

    tracing::info!("call data {data:#?}");

    let integration = parts
        .extensions
        .get::<DioxusIntegrationDyn>()
        .ok_or_else(|| anyhow!("Dioxus Shield integration should be extracted."))?;
    let shield = integration.extract_shield(&parts.extensions)?;
    let session = integration.extract_session(&parts.extensions)?;

    let response = shield
        .call(
            &action_id,
            &method_id,
            provider_id.as_deref(),
            session,
            Request {
                query: Value::Null,
                form_data: data,
            },
        )
        .await
        .context("Failed to call Shield action.")?;

    Ok(response)
}
