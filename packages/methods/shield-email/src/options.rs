use std::sync::Arc;

use bon::Builder;
use chrono::TimeDelta;
use secrecy::SecretString;

use crate::sender::Sender;

#[derive(Builder, Clone)]
#[builder(on(String, into), state_mod(vis = "pub(crate)"))]
pub struct EmailOptions {
    #[builder(into)]
    pub(crate) secret: SecretString,

    #[builder(with = |sender: impl Sender + 'static| Arc::new(sender))]
    pub(crate) sender: Arc<dyn Sender>,

    #[builder(default = TimeDelta::minutes(10))]
    pub(crate) expires_in: TimeDelta,

    #[builder(default = "/")]
    pub(crate) sign_in_redirect: String,
}
