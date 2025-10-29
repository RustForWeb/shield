use std::sync::Arc;

use anyhow::Result;
use dioxus::fullstack::http::Extensions;
use shield::{Session, ShieldDyn};

pub trait DioxusIntegration: Send + Sync {
    fn extract_shield(&self, extensions: &Extensions) -> Result<ShieldDyn>;

    fn extract_session(&self, extensions: &Extensions) -> Result<Session>;
}

#[derive(Clone)]
pub struct DioxusIntegrationDyn(Arc<dyn DioxusIntegration>);

impl DioxusIntegrationDyn {
    pub fn new<I: DioxusIntegration + 'static>(integration: I) -> Self {
        Self(Arc::new(integration))
    }

    pub fn extract_shield(&self, extensions: &Extensions) -> Result<ShieldDyn> {
        self.0.extract_shield(extensions)
    }

    pub fn extract_session(&self, extensions: &Extensions) -> Result<Session> {
        self.0.extract_session(extensions)
    }
}
