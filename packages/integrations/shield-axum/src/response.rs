use axum::response::{IntoResponse, Redirect, Response};

pub struct RouteResponse(shield::Response);

impl IntoResponse for RouteResponse {
    fn into_response(self) -> Response {
        match self.0 {
            shield::Response::Redirect(url) => Redirect::to(&url).into_response(),
        }
    }
}

impl From<shield::Response> for RouteResponse {
    fn from(value: shield::Response) -> Self {
        Self(value)
    }
}
