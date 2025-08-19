use dioxus::prelude::*;
use serde_json::Value;
use shield::{ActionForms, Response};

use crate::ErasedDioxusStyle;

#[derive(Clone, PartialEq, Props)]
pub struct ActionProps {
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

// TODO: Figure out a way to access `FromContext` and `extract` without `dioxus/server` feature.

#[cfg_attr(not(feature = "server"), allow(unused_variables))]
#[server]
async fn forms(action_id: String) -> Result<ActionForms, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use dioxus::prelude::{FromContext, extract};

        use crate::integration::DioxusIntegrationDyn;

        let FromContext(integration): FromContext<DioxusIntegrationDyn> = extract().await?;
        let shield = integration.extract_shield().await;
        let session = integration.extract_session().await;

        let forms = shield.action_forms(&action_id, session).await?;

        Ok(forms)
    }

    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[cfg_attr(not(feature = "server"), allow(unused_variables))]
#[server]
pub async fn call(
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
    // TODO: Would be nice if this argument could fill up with all unknown keys instead of setting name to `data[...]`.
    data: Value,
) -> Result<Response, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use dioxus::prelude::{FromContext, extract};
        use serde_json::Value;
        use shield::{Request, Response};

        use crate::integration::DioxusIntegrationDyn;

        tracing::info!("call data {data:#?}");

        let FromContext(integration): FromContext<DioxusIntegrationDyn> = extract().await?;
        let shield = integration.extract_shield().await;
        let session = integration.extract_session().await;

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
            .await?;

        Ok(response)
    }

    #[cfg(not(feature = "server"))]
    unreachable!()
}
