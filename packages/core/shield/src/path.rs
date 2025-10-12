use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct ActionFormsPathParams {
    /// ID of the action.
    pub action_id: String,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct ActionPathParams {
    /// ID of the method.
    pub method_id: String,
    /// ID of the action.
    pub action_id: String,
    /// ID of provider (optional).
    pub provider_id: Option<String>,
}
