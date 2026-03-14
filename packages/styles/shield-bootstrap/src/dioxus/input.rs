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
                    InputValue::Origin => "TODO: origin".to_owned(),
                    InputValue::Query { key } => {
                        query.get(&key).cloned().unwrap_or_default()
                    },
                    InputValue::String { value } => value.clone(),
                }),
                placeholder: props.input.label,
            }
        }
    }
}
