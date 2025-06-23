use shield::Provider;

use crate::CREDENTIALS_METHOD_ID;

pub struct CredentialsProvider;

impl Provider for CredentialsProvider {
    fn method_id(&self) -> String {
        CREDENTIALS_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "Credentials".to_owned()
    }
}
