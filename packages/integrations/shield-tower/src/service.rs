use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use http::{Request, Response};
use shield::{Session, Shield};
use tower_service::Service;

use crate::session::TowerSessionStorage;

#[derive(Clone)]
pub struct ShieldService<S> {
    inner: S,
    shield: Shield,
    session_key: &'static str,
}

impl<S> ShieldService<S> {
    pub fn new(inner: S, shield: Shield, session_key: &'static str) -> Self {
        Self {
            inner,
            shield,
            session_key,
        }
    }

    fn internal_server_error<ResBody: Default>() -> Response<ResBody> {
        let mut response = Response::default();
        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
        response
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ShieldService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // TODO: Improve error handling to not only return a 500 response.

        //  https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let shield = self.shield.clone();
        let session_key = self.session_key;

        Box::pin(async move {
            let session = match req.extensions().get::<tower_sessions::Session>() {
                Some(session) => session,
                None => {
                    return Ok(Self::internal_server_error());
                }
            };

            let session_storage =
                match TowerSessionStorage::load(session.clone(), session_key).await {
                    Ok(session_storage) => session_storage,
                    Err(_err) => return Ok(Self::internal_server_error()),
                };
            let shield_session = Session::new(session_storage);

            req.extensions_mut().insert(shield);
            req.extensions_mut().insert(shield_session);

            inner.call(req).await
        })
    }
}
