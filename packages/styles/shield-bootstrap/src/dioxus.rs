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

                for form in &action.forms {
                    Form {
                        action_id: action.id.clone(),
                        form: form.clone(),
                    }
                }
            }
        }
    }
}
