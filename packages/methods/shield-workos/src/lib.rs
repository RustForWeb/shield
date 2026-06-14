mod actions;
mod method;
mod options;
mod provider;

pub use method::*;
pub use options::*;

#[doc(no_inline)]
pub use workos::{Client as Workos, UserManagementAuthenticationProvider as WorkosOauthProvider};

// TODO: Support both AuthKit method and self hosted method.
