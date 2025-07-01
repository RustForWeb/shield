use leptos::prelude::*;
use shield::{ActionForms, Input};
use shield_leptos::{ErasedLeptosStyle, LeptosStyle};

#[derive(Default)]
pub struct BootstrapLeptosStyle {}

impl BootstrapLeptosStyle {
    pub fn context(self) -> ErasedLeptosStyle {
        ErasedLeptosStyle::new(self)
    }
}

impl BootstrapLeptosStyle {
    fn render_form_input(&self, input: &Input) -> impl IntoView {
        view! {
            <div class="mb-3">
                {self.render_label(input)}
                {self.render_input(input)}
            </div>
        }
    }

    fn render_label(&self, input: &Input) -> Option<impl IntoView> {
        input.label.as_ref().map(|label| {
            view! {
                <label class="form-label">{label.clone()}</label>
            }
        })
    }

    fn render_input(&self, input: &Input) -> impl IntoView {
        view! {
            <input
                class="form-control"
                name=input.name.clone()
                r#type=input.r#type.as_str()
                value=input.value.clone()
            />
        }
    }
}

impl LeptosStyle for BootstrapLeptosStyle {
    fn render(&self, action: &ActionForms) -> AnyView {
        view! {
            <div class="container">
                <h1>{action.name.clone()}</h1>

                {action.forms.iter().map(|form| view! {
                    <form>
                        {form.inputs.iter().map(|input| self.render_form_input(input)).collect_view()}
                    </form>
                }).collect_view()}
            </div>
        }
        .into_any()
    }
}
