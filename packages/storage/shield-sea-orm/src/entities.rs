pub mod prelude;

pub mod email_address;
pub mod user;

#[cfg(feature = "entity")]
pub mod entity;

#[cfg(feature = "provider-email")]
pub mod email_auth_token;

#[cfg(feature = "provider-oauth")]
pub mod oauth_provider;
#[cfg(feature = "provider-oauth")]
pub mod oauth_provider_connection;

#[cfg(feature = "provider-oidc")]
pub mod oidc_provider;
#[cfg(feature = "provider-oidc")]
pub mod oidc_provider_connection;

// TODO: Use features to ensure all databases are supported (e.g. for enums).
