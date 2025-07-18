use dioxus::prelude::*;
use shield::ActionForms;

use crate::{DioxusIntegrationDyn, ErasedDioxusStyle};

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

#[server]
async fn forms(action_id: String) -> Result<ActionForms, ServerFnError> {
    let FromContext(integration): FromContext<DioxusIntegrationDyn> = extract().await?;
    let shield = integration.extract_shield().await;
    let session = integration.extract_session().await;

    let forms = shield.action_forms(&action_id, session).await?;

    Ok(forms)
}
