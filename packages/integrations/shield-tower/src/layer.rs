use shield::{Shield, User};
use tower_layer::Layer;

use crate::service::ShieldService;

pub const SESSION_KEY: &str = "shield";

#[derive(Clone)]
pub struct ShieldLayer<U: User> {
    shield: Shield<U>,
    session_key: &'static str,
}

impl<U: User> ShieldLayer<U> {
    pub fn new(shield: Shield<U>) -> Self {
        Self::new_with_session_key(shield, SESSION_KEY)
    }

    pub fn new_with_session_key(shield: Shield<U>, session_key: &'static str) -> Self {
        Self {
            shield,
            session_key,
        }
    }
}

impl<S, U: User + Clone> Layer<S> for ShieldLayer<U> {
    type Service = ShieldService<S, U>;

    fn layer(&self, inner: S) -> Self::Service {
        ShieldService::new(inner, self.shield.clone(), self.session_key)
    }
}
