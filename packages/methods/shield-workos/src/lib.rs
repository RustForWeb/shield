mod actions;
mod client;
mod method;
mod options;
mod provider;

pub use method::*;
pub use options::*;

#[doc(no_inline)]
pub use workos_sdk::{
    ApiKey as WorkosApiKey, WorkOs as Workos, WorkOsBuilder as WorkosBuilder,
    user_management::OauthProvider as WorkosOauthProvider,
};

// TODO: Support both AuthKit method and self hosted method.
