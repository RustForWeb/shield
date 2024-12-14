use std::{
    sync::Arc,
    task::{Context, Poll},
};

use http::{Request, Response};
use shield::Shield;
use tower_service::Service;

#[derive(Clone)]
pub struct ShieldService<S> {
    inner: S,
    shield: Arc<Shield>,
}

impl<S> ShieldService<S> {
    pub fn new(inner: S, shield: Arc<Shield>) -> Self {
        Self { inner, shield }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ShieldService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.extensions_mut().insert(self.shield.clone());
        self.inner.call(req)
    }
}
