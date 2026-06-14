use dioxus::prelude::*;
use shield::{Input, InputType, InputValue};
use shield_dioxus::Query;

use crate::dioxus::input_addon::FormInputAddon;

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

    let value = props.input.value.and_then(|value| match value {
        InputValue::Origin => origin(),
        InputValue::Query { key } => query.get(&key).cloned(),
        InputValue::String { value } => Some(value.clone()),
    });

    let mut element = match props.input.r#type {
        InputType::Button(_) | InputType::Reset(_) | InputType::Submit(_) => {
            return rsx! {
                button {
                    class: "btn btn-outline-primary d-flex align-items-center justify-content-center gap-1",
                    name: props.input.name,
                    type: match props.input.r#type {
                        InputType::Reset(_) => "reset",
                        InputType::Submit(_) => "submit",
                        _ => "button"
                    },

                    if let Some(addon) = props.input.addon_start {
                        FormInputAddon {
                            addon,
                            group: false,
                        }
                    }

                    {value}

                    if let Some(addon) = props.input.addon_end {
                        FormInputAddon {
                            addon,
                            group: false,
                        }
                    }
                }
            };
        }
        _ => {
            rsx! {
                input {
                    class: "form-control",
                    name: props.input.name,
                    type: props.input.r#type.as_str(),
                    value,
                    placeholder: props.input.label.clone(),
                }
            }
        }
    };

    if matches!(props.input.r#type, InputType::Hidden(_)) {
        return element;
    }

    if props.input.addon_start.is_some() || props.input.addon_end.is_some() {
        element = rsx! {
            div {
                class: "input-group",

                if let Some(addon) = props.input.addon_start {
                    FormInputAddon {
                        addon,
                        group: true,
                    }
                }

                { element }

                if let Some(addon) = props.input.addon_end {
                    FormInputAddon {
                        addon,
                        group: true,
                    }
                }
            }
        }
    }

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

            {element}
        }
    }
}
