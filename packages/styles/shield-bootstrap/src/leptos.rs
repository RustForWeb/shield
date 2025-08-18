mod form;
mod input;

use leptos::prelude::*;
use shield::ActionForms;
use shield_leptos::{ErasedLeptosStyle, LeptosStyle};

use crate::leptos::form::Form;

#[derive(Default)]
pub struct BootstrapLeptosStyle {}

impl BootstrapLeptosStyle {
    pub fn context(self) -> ErasedLeptosStyle {
        ErasedLeptosStyle::new(self)
    }
}

impl LeptosStyle for BootstrapLeptosStyle {
    fn render(&self, action: &ActionForms) -> AnyView {
        view! {
            <div class="container">
                <h1>{action.name.clone()}</h1>

                {action.forms.iter().map(|form| view! {
                    <Form action_id=action.id.clone() form=form.clone() />
                }).collect_view()}
            </div>
        }
        .into_any()
    }
}
