use shield::Provider;

use crate::method::WORKOS_METHOD_ID;

pub struct WorkosProvider;

impl Provider for WorkosProvider {
    fn method_id(&self) -> String {
        WORKOS_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "WorkOS".to_owned()
    }
}
