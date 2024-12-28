use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use leptos::prelude::provide_context;
use leptos_actix::{extract, redirect};
use shield::{Session, ShieldDyn, User};
use shield_actix::{ExtractSession, ExtractShield, ExtractUser};
use shield_leptos::integration::{LeptosIntegration, LeptosUser};

pub struct LeptosActixIntegration<U: User>(PhantomData<U>);

impl<U: User> Default for LeptosActixIntegration<U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<U: User + Clone + 'static> LeptosIntegration for LeptosActixIntegration<U> {
    async fn extract_shield(&self) -> ShieldDyn {
        let ExtractShield(shield) = extract::<ExtractShield<U>>().await.expect("TOD");

        ShieldDyn::new(shield)
    }

    async fn extract_session(&self) -> Session {
        let ExtractSession(session) = extract().await.expect("TODO");

        session
    }

    async fn extract_user(&self) -> Option<LeptosUser> {
        let ExtractUser(user) = extract::<ExtractUser<U>>().await.expect("TODO");

        user.map(|user| user.into())
    }

    fn redirect(&self, path: &str) {
        redirect(path);
    }
}

pub fn provide_actix_integration<U: User + Clone + 'static>() {
    provide_context::<Arc<dyn LeptosIntegration>>(Arc::new(LeptosActixIntegration::<U>::default()));
}
