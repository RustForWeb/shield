use shield::Provider;

use crate::method::EMAIL_METHOD_ID;

pub struct EmailProvider;

impl Provider for EmailProvider {
    fn method_id(&self) -> String {
        EMAIL_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "Email".to_owned()
    }
}
