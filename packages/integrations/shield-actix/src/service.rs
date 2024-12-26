use std::task::{Context, Poll};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use shield::{Shield, User};

pub struct ShieldService<S, U: User> {
    inner: S,
    shield: Shield<U>,
}

impl<S, U: User> ShieldService<S, U> {
    pub fn new(inner: S, shield: Shield<U>) -> Self {
        Self { inner, shield }
    }
}

impl<S, U: User + Clone + 'static, ResBody> Service<ServiceRequest> for ShieldService<S, U>
where
    S: Service<ServiceRequest, Response = ServiceResponse<ResBody>, Error = Error>,
{
    type Response = ServiceResponse<ResBody>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.extensions_mut().insert(self.shield.clone());
        self.inner.call(req)
    }
}
