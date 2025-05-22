use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInRequest {
    pub method_id: String,
    pub provider_id: Option<String>,
    pub redirect_url: Option<String>,
    pub data: Option<Value>,
    pub form_data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInCallbackRequest {
    pub method_id: String,
    pub provider_id: Option<String>,
    pub redirect_url: Option<String>,
    pub query: Option<Value>,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOutRequest {
    pub method_id: String,
    pub provider_id: Option<String>,
}
