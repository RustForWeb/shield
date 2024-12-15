use actix_utils::future::{ready, Ready};
use actix_web::{
    dev::Payload, error::ErrorInternalServerError, Error, FromRequest, HttpMessage, HttpRequest,
};
use shield::Shield;

pub struct ExtractShield(pub Shield);

impl FromRequest for ExtractShield {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<Shield>()
                .cloned()
                .map(ExtractShield)
                .ok_or(ErrorInternalServerError(
                    "Can't extract Shield. Is `ShieldTransform` enabled?",
                )),
        )
    }
}
