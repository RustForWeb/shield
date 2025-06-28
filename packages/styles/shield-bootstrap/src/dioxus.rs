use dioxus::prelude::*;
use shield::Form;
use shield_dioxus::{DioxusStyle, ErasedDioxusStyle};

#[derive(Default)]
pub struct BootstrapStyle {}

impl BootstrapStyle {
    pub fn context(self) -> ErasedDioxusStyle {
        ErasedDioxusStyle::new(self)
    }
}

impl DioxusStyle for BootstrapStyle {
    fn render(&self, forms: &[Form]) -> Element {
        rsx! {
            div {
                class: "container",

                h1 {
                    // TODO: Get from action.
                    "Sign in"
                }

                for form in forms {
                    form {
                        for input in &form.inputs {
                            div {
                                class: "mb-3",

                                if let Some(label) = &input.label {
                                    label {
                                        class: "form-label",

                                        strong {
                                            "{label}"
                                        }
                                    }
                                }

                                input {
                                    class: "form-control",
                                    name: input.name.clone(),
                                    type: input.r#type.as_str(),
                                    value: input.value.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
