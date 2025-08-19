use dioxus::{logger::tracing::info, prelude::*};
use shield::Response;
use shield_dioxus::call;

use crate::dioxus::input::FormInput;

#[derive(Clone, PartialEq, Props)]
pub struct FormProps {
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
    form: shield::Form,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    let navigator = navigator();

    rsx! {
        form {
            onsubmit: {
                move |event| {
                    let action_id = props.action_id.clone();
                    let method_id = props.method_id.clone();
                    let provider_id = props.provider_id.clone();

                    event.prevent_default();

                    async move {
                        info!("{:?}", event);
                        // TODO: Replace `expect` with proper error handling.
                        let data = serde_json::to_value(event.data().values()).expect("Valid JSON.");

                        let result = call(action_id, method_id, provider_id, data).await;
                        info!("{:?}", result);

                        // TODO: Handle error.
                        if let Ok(response) = result {
                            match response {
                                Response::Default => todo!("default response"),
                                Response::Redirect(to) => {
                                    navigator.push(to);
                                },
                            }
                        }
                    }
                }
            },

            for input in props.form.inputs {
                FormInput {
                    input: input
                }
            }
        }
    }
}
