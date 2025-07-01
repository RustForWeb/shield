use std::sync::Arc;

use dioxus::prelude::Element;
use shield::ActionForms;

pub trait DioxusStyle: Send + Sync {
    fn render(&self, action: &ActionForms) -> Element;
}

#[derive(Clone)]
pub struct ErasedDioxusStyle(Arc<dyn DioxusStyle>);

impl ErasedDioxusStyle {
    pub fn new<I: DioxusStyle + 'static>(integration: I) -> Self {
        Self(Arc::new(integration))
    }

    pub fn render(&self, action: &ActionForms) -> Element {
        self.0.render(action)
    }
}
