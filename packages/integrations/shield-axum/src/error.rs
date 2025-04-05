use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use shield::{ShieldError, StorageError};

#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(as = Error, examples(
    json!({
        "status_code": 500,
        "status_reason": "Internal Server Error",
        "message": "Unknown"
    })
)))]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    status_code: u16,
    status_reason: Option<String>,
    message: String,
}

impl ErrorBody {
    fn new(status_code: StatusCode, error: ShieldError) -> Self {
        Self {
            status_code: status_code.as_u16(),
            status_reason: status_code.canonical_reason().map(ToOwned::to_owned),
            message: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct RouteError(ShieldError);

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        let status_code = match &self.0 {
            ShieldError::Provider(provider_error) => match provider_error {
                shield::ProviderError::ProviderNotFound(_) => StatusCode::NOT_FOUND,
                shield::ProviderError::SubproviderMissing => StatusCode::BAD_REQUEST,
                shield::ProviderError::SubproviderNotFound(_) => StatusCode::NOT_FOUND,
            },
            ShieldError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShieldError::Session(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShieldError::Storage(storage_error) => match storage_error {
                StorageError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
                StorageError::Validation(_) => StatusCode::BAD_REQUEST,
                StorageError::NotFound(_, _) => StatusCode::NOT_FOUND,
                StorageError::Engine(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ShieldError::Request(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShieldError::Validation(_) => StatusCode::BAD_REQUEST,
            ShieldError::Unauthorized => StatusCode::UNAUTHORIZED,
        };

        (status_code, Json(ErrorBody::new(status_code, self.0))).into_response()
    }
}

impl From<ShieldError> for RouteError {
    fn from(value: ShieldError) -> Self {
        Self(value)
    }
}
