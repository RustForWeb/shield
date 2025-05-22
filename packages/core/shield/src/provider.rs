use serde::{Deserialize, Serialize};

use crate::form::Form;

pub trait Provider: Send + Sync {
    fn method_id(&self) -> String;

    fn id(&self) -> Option<String>;

    fn name(&self) -> String;

    fn icon_url(&self) -> Option<String>;

    fn form(&self) -> Option<Form>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProviderVisualisation {
    pub key: String,
    pub method_id: String,
    pub provider_id: Option<String>,
    pub name: String,
    pub icon_url: Option<String>,
}
