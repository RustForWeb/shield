use std::marker::PhantomData;

use anyhow::{Result, anyhow};
use dioxus_server::http::Extensions;
use shield::{Session, Shield, ShieldDyn, User};
use shield_axum::{ExtractSession, ExtractShield};
use shield_dioxus::{DioxusIntegration, DioxusIntegrationDyn};

pub struct AxumDioxusIntegration<U: User>(PhantomData<U>);

impl<U: User + Clone + 'static> AxumDioxusIntegration<U> {
    pub fn context(self) -> DioxusIntegrationDyn {
        DioxusIntegrationDyn::new(self)
    }
}

impl<U: User> Default for AxumDioxusIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<U: User + Clone + 'static> DioxusIntegration for AxumDioxusIntegration<U> {
    fn extract_shield(&self, extensions: &Extensions) -> Result<ShieldDyn> {
        let ExtractShield(shield) = extensions
            .get::<Shield<U>>()
            .cloned()
            .map(ExtractShield)
            .ok_or_else(|| anyhow!("Shield should be extracted"))?;

        Ok(ShieldDyn::new(shield))
    }

    fn extract_session(&self, extensions: &Extensions) -> Result<Session> {
        let ExtractSession(session) = extensions
            .get::<Session>()
            .cloned()
            .map(ExtractSession)
            .ok_or_else(|| anyhow!("Session should be extracted"))?;

        Ok(session)
    }
}
