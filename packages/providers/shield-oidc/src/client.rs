use openidconnect::reqwest::{self, redirect::Policy};
use shield::ConfigurationError;

pub fn async_http_client() -> Result<reqwest::Client, ConfigurationError> {
    reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .map_err(|err| ConfigurationError::Invalid(err.to_string()))
}
