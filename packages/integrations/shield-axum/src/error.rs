use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shield::ShieldError;

pub struct RouteError(ShieldError);

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0)).into_response()
    }
}

impl From<ShieldError> for RouteError {
    fn from(value: ShieldError) -> Self {
        Self(value)
    }
}
