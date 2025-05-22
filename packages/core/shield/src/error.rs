use thiserror::Error;

#[derive(Debug, Error)]
pub enum MethodError {
    #[error("method `{0}` not found")]
    MethodNotFound(String),
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider is missing")]
    ProviderMissing,
    #[error("provider `{0}` not found")]
    ProviderNotFound(String),
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
    Method(#[from] MethodError),
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
