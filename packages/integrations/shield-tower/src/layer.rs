use shield::Shield;
use tower_layer::Layer;

use crate::service::ShieldService;

pub const SESSION_KEY: &str = "shield";

#[derive(Clone)]
pub struct ShieldLayer {
    shield: Shield,
    session_key: &'static str,
}

impl ShieldLayer {
    pub fn new(shield: Shield) -> Self {
        Self::new_with_session_key(shield, SESSION_KEY)
    }

    pub fn new_with_session_key(shield: Shield, session_key: &'static str) -> Self {
        Self {
            shield,
            session_key,
        }
    }
}

impl<S> Layer<S> for ShieldLayer {
    type Service = ShieldService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ShieldService::new(inner, self.shield.clone(), self.session_key)
    }
}
