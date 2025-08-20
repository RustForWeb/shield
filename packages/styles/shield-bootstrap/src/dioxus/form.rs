use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use shield::Response;
use shield_dioxus::{ShieldRouter, call};

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
                        let data = serde_json::to_value(
                            // TODO: Support inputs with `multiple` attribute.
                            event
                                .data()
                                .values()
                                .into_iter()
                                .filter_map(|(key, values)| values.first().map(|value| (key, value.clone())))
                                .collect::<HashMap<String, String>>()
                        ).expect("TODO: handle error");

                        let result = call(action_id, method_id, provider_id, data).await;
                        info!("{:?}", result);

                        // TODO: Handle error.
                        if let Ok(response) = result {
                            match response {
                                Response::Default => todo!("default response"),
                                Response::Redirect(to) => {
                                    navigator.push(to);
                                },
                                Response::RedirectToAction { action_id } => {
                                    navigator.push(ShieldRouter::Action { action_id });
                                }
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
