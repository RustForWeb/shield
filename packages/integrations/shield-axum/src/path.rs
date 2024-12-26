use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthPath {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
}
