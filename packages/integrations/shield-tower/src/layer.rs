use shield::Shield;
use tower_layer::Layer;

use crate::service::ShieldService;

#[derive(Clone)]
pub struct ShieldLayer {
    shield: Shield,
}

impl ShieldLayer {
    pub fn new(shield: Shield) -> Self {
        Self { shield }
    }
}

impl<S> Layer<S> for ShieldLayer {
    type Service = ShieldService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ShieldService::new(inner, self.shield.clone())
    }
}
