use dioxus::prelude::*;
use shield::Input;

#[derive(Clone, PartialEq, Props)]
pub struct FormInputProps {
    input: Input,
}

#[component]
pub fn FormInput(props: FormInputProps) -> Element {
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
                value: props.input.value.clone(),
            }
        }
    }
}
