use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider `{0}` not found")]
    ProviderNotFound(String),
    #[error("subprovider is missing")]
    SubproviderMissing,
    #[error("subprovider `{0}` not found")]
    SubproviderNotFound(String),
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("missing configuration: {0}")]
    Missing(String),
    #[error("invalid configuration: {0}")]
    Invalid(String),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Configuration(#[from] ConfigurationError),
    #[error("{0}")]
    Validation(String),
    #[error("{0} with ID `{1}` not found.")]
    NotFound(String, String),
    #[error("{0}")]
    Engine(String),
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error(transparent)]
    Configuration(#[from] ConfigurationError),
    #[error("{0}")]
    Engine(String),
    #[error("{0}")]
    Lock(String),
    #[error("{0}")]
    Serialization(String),
}

#[derive(Debug, Error)]

pub enum ShieldError {
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Configuration(#[from] ConfigurationError),
    #[error(transparent)]
    Session(#[from] SessionError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("{0}")]
    Request(String),
    #[error("{0}")]
    Validation(String),
    #[error("Unauthorized")]
    Unauthorized,
}
