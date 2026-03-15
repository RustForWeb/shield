use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use shield::ResponseType;
use shield_dioxus::{ShieldRouter, call_method};

use crate::dioxus::input::FormInput;

#[derive(Clone, PartialEq, Props)]
pub struct MethodFormProps {
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
    form: shield::Form,
}

#[component]
pub fn MethodForm(props: MethodFormProps) -> Element {
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
                                .filter_map(|(key, value)| match value {
                                    FormValue::Text(value) => Some((key, value)),
                                    FormValue::File(_) => None,
                                })
                                .collect::<HashMap<String, String>>()
                        ).expect("TODO: handle error");

                        let result = call_method(action_id, method_id, provider_id, data).await;

                        match result {
                            Ok(response) => {
                                info!("{:?}", response);

                                match response {
                                    ResponseType::Default => {},
                                    ResponseType::Redirect(to) => {
                                        navigator.push(to);
                                    },
                                    ResponseType::RedirectToAction { action_id } => {
                                        navigator.push(ShieldRouter::Action { action_id, query: "".to_owned() });
                                    }
                                }
                            }
                            Err(err) => {
                                // TODO: Handle error.
                                error!("{err}");
                            }
                        }
                    }
                }
            },

            for input in props.form.inputs {
                FormInput {
                    input: input,
                }
            }
        }
    }
}
