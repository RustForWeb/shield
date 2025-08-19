mod index;
mod sign_in;
mod sign_out;
mod sign_up;

pub use index::*;
pub use sign_in::*;
pub use sign_out::*;
pub use sign_up::*;

// TODO:
// - Index action
//     - Email form
//     - SSO button forms
// - Sign in action
//     - Password form
//     - Magic auth button
//     - SSO button forms (if enabled in options only show the ones the user has connected)
// - Sign up action
//     - Password form
//     - Magic auth button
//     - SSO button forms
// - SSO callback action
// - Email verification action
// - Magic auth action
// - Forgot password action
// - Reset password action
// - MFA challenge action
// - MFA enrollment action
