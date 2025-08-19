mod form;
mod input;

use dioxus::prelude::*;
use shield::ActionForms;
use shield_dioxus::{DioxusStyle, ErasedDioxusStyle};

use crate::dioxus::form::Form;

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

                for method_form in &action.method_forms {
                    for provider_form in &method_form.provider_forms {
                        Form {
                            action_id: action.id.clone(),
                            method_id: method_form.id.clone(),
                            provider_id: provider_form.id.clone(),
                            form: provider_form.form.clone(),
                        }
                    }
                }
            }
        }
    }
}
