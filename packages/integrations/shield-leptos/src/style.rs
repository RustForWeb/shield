use std::sync::Arc;

use leptos::prelude::AnyView;
use shield::ActionForms;

pub trait LeptosStyle: Send + Sync {
    fn render(&self, action: &ActionForms) -> AnyView;
}

#[derive(Clone)]
pub struct ErasedLeptosStyle(Arc<dyn LeptosStyle>);

impl ErasedLeptosStyle {
    pub fn new<I: LeptosStyle + 'static>(integration: I) -> Self {
        Self(Arc::new(integration))
    }

    pub fn render(&self, action: &ActionForms) -> AnyView {
        self.0.render(action)
    }
}
