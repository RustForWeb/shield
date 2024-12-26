use shield::User;

use crate::ExtractShield;

pub async fn sign_out<U: User>(ExtractShield(_shield): ExtractShield<U>) {}
