use dioxus::{logger::tracing::info, prelude::*};
use shield::ActionProviderForm;
use shield_dioxus::call;

use crate::dioxus::input::FormInput;

#[derive(Clone, PartialEq, Props)]
pub struct FormProps {
    action_id: String,
    form: ActionProviderForm,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        form {
            onsubmit: {
                move |event| {
                    let action_id = props.action_id.clone();
                    let method_id = props.form.method_id.clone();
                    let provider_id = props.form.provider_id.clone();

                    event.prevent_default();

                    async move {
                        info!("{:?}", event);

                        let result = call(action_id, method_id, provider_id).await;
                        info!("{:?}", result);
                    }
                }
            },

            for input in props.form.form.inputs {
                FormInput {
                    input: input
                }
            }
        }
    }
}
