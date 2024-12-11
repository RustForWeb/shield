pub use super::email_address::Entity as EmailAddress;
pub use super::user::Entity as User;

#[cfg(feature = "entity")]
pub use super::entity::Entity;

#[cfg(feature = "provider-email")]
pub use super::email_auth_token::Entity as EmailAuthToken;
