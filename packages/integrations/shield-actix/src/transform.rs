use actix_utils::future::{ready, Ready};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use shield::{Shield, User};

use crate::service::ShieldService;

// Actix uses a `Middleware` suffix instead of a `Transform` suffix, despite the trait being called `Transform`.
// Export both names so users can choose.
pub type ShieldMiddleware<U> = ShieldTransform<U>;

pub struct ShieldTransform<U: User> {
    shield: Shield<U>,
}

impl<U: User> ShieldTransform<U> {
    pub fn new(shield: Shield<U>) -> Self {
        Self { shield }
    }
}

impl<S, U: User + Clone + 'static, ResBody> Transform<S, ServiceRequest> for ShieldTransform<U>
where
    S: Service<ServiceRequest, Response = ServiceResponse<ResBody>, Error = Error>,
{
    type Response = ServiceResponse<ResBody>;
    type Error = Error;
    type Transform = ShieldService<S, U>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, inner: S) -> Self::Future {
        ready(Ok(ShieldService::new(inner, self.shield.clone())))
    }
}
