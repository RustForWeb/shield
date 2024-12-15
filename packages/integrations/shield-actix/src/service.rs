use std::task::{Context, Poll};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use shield::Shield;

pub struct ShieldService<S> {
    inner: S,
    shield: Shield,
}

impl<S> ShieldService<S> {
    pub fn new(inner: S, shield: Shield) -> Self {
        Self { inner, shield }
    }
}

impl<S, ResBody> Service<ServiceRequest> for ShieldService<S>
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
