mod form;
mod input;
mod method_form;

use leptos::prelude::*;
use shield::ActionForms;
use shield_leptos::{ErasedLeptosStyle, LeptosStyle};

use crate::leptos::{form::Form, method_form::MethodForm};

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
                    <Form
                        action_id=action.id.clone()
                        form=form.clone()
                    />
                }).collect_view()}

                {action.method_forms.iter().flat_map(|method_form| method_form.provider_forms.iter().map(|provider_form| view! {
                    <MethodForm
                        action_id=action.id.clone()
                        method_id=method_form.id.clone()
                        provider_id=provider_form.id.clone()
                        form=provider_form.form.clone()
                    />
                })).collect_view()}
            </div>
        }
        .into_any()
    }
}
