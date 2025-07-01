use dioxus::prelude::*;
use shield::ActionForms;
use shield_dioxus::{DioxusStyle, ErasedDioxusStyle};

#[derive(Default)]
pub struct BootstrapDioxusStyle {}

impl BootstrapDioxusStyle {
    pub fn context(self) -> ErasedDioxusStyle {
        ErasedDioxusStyle::new(self)
    }
}

impl DioxusStyle for BootstrapDioxusStyle {
    fn render(&self, action: &ActionForms) -> Element {
        rsx! {
            div {
                class: "container",

                h1 {
                    "{action.name}"
                }

                for form in &action.forms {
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
