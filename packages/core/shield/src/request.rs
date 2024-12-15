use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::storage::StorageError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignInRequest {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
    pub data: Option<Value>,
    pub form_data: Option<Value>,
}

#[derive(Debug, Error)]

pub enum SignInError {
    #[error("provider `{0}` not found")]
    ProviderNotFound(String),
    #[error("subprovider `{0}` not found")]
    SubproviderNotFound(String),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOutRequest {
    pub provider_id: String,
    pub subprovider_id: Option<String>,
}

#[derive(Debug, Error)]

pub enum SignOutError {
    #[error("provider `{0}` not found")]
    ProviderNotFound(String),
    #[error("subprovider `{0}` not found")]
    SubproviderNotFound(String),
    #[error(transparent)]
    Storage(#[from] StorageError),
}
