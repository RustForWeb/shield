use thiserror::Error;

#[derive(Debug, Error)]
pub enum MethodError {
    #[error("method `{0}` not found")]
    NotFound(String),
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("action `{0}` not found")]
    NotFound(String),
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider is missing")]
    Missing,
    #[error("{}", provider_not_found_message(.0))]
    NotFound(Option<String>),
}

fn provider_not_found_message(provider_id: &Option<String>) -> String {
    match provider_id {
        Some(id) => format!("provider `{id}` not found"),
        None => "provider not found".to_owned(),
    }
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
    Action(#[from] ActionError),
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
