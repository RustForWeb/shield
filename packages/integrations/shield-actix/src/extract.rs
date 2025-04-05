use actix_utils::future::{Ready, ready};
use actix_web::{
    Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, error::ErrorInternalServerError,
};
use shield::{Session, Shield, User};

pub struct ExtractShield<U: User>(pub Shield<U>);

impl<U: User + Clone + 'static> FromRequest for ExtractShield<U> {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<Shield<U>>()
                .cloned()
                .map(ExtractShield)
                .ok_or(ErrorInternalServerError(
                    "Can't extract Shield. Is `ShieldTransform` enabled?",
                )),
        )
    }
}

pub struct ExtractSession(pub Session);

impl FromRequest for ExtractSession {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<Session>()
                .cloned()
                .map(ExtractSession)
                .ok_or(ErrorInternalServerError(
                    "Can't extract Shield session. Is `ShieldTransform` enabled?",
                )),
        )
    }
}

pub struct ExtractUser<U: User>(pub Option<U>);

impl<U: User + Clone + 'static> FromRequest for ExtractUser<U> {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<Option<U>>()
                .cloned()
                .map(ExtractUser)
                .ok_or(ErrorInternalServerError(
                    "Can't extract Shield user. Is `ShieldTransform` enabled?",
                )),
        )
    }
}
