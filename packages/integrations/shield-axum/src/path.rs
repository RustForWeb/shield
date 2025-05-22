use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct AuthPathParams {
    /// ID of authentication method.
    pub method_id: String,
    /// ID of authentication provider (optional).
    pub provider_id: Option<String>,
}
