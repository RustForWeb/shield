use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct AuthPathParams {
    /// ID of authentication provider.
    pub provider_id: String,
    /// ID of authentication subprovider (optional).
    pub subprovider_id: Option<String>,
}
