pub mod prelude;

pub mod email_address;
pub mod user;

#[cfg(feature = "entity")]
pub mod entity;

#[cfg(feature = "method-email")]
pub mod email_auth_token;

#[cfg(feature = "method-oauth")]
pub mod oauth_provider;
#[cfg(feature = "method-oauth")]
pub mod oauth_provider_connection;

#[cfg(feature = "method-oidc")]
pub mod oidc_provider;
#[cfg(feature = "method-oidc")]
pub mod oidc_provider_connection;
