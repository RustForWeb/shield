use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInRequest {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
    pub data: Option<Value>,
    pub form_data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInCallbackRequest {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
    pub query: Option<Value>,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOutRequest {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
}
