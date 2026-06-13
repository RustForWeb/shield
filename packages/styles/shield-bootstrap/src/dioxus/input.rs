use dioxus::prelude::*;
use shield::{Input, InputValue};
use shield_dioxus::Query;

#[derive(Clone, PartialEq, Props)]
pub struct FormInputProps {
    input: Input,
}

#[component]
pub fn FormInput(props: FormInputProps) -> Element {
    let query = use_context::<Query>();

    #[cfg_attr(not(target_arch = "wasm32"), expect(unused_mut))]
    let mut origin = use_signal(|| None::<String>);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        origin.set(web_sys::window().and_then(|window| window.location().origin().ok()))
    });

    rsx! {
        div {
            class: "mb-3",

            if let Some(label) = &props.input.label {
                label {
                    class: "form-label",

                    strong {
                        "{label}"
                    }
                }
            }

            input {
                class: "form-control",
                name: props.input.name,
                type: props.input.r#type.as_str(),
                value: props.input.value.map(|value| match value {
                    InputValue::Origin => origin(),
                    InputValue::Query { key } => {
                        query.get(&key).cloned()
                    },
                    InputValue::String { value } => Some(value.clone()),
                }),
                placeholder: props.input.label,
            }
        }
    }
}
