use shield::Provider;

use crate::method::DUMMY_METHOD_ID;

pub struct DummyProvider;

impl Provider for DummyProvider {
    fn method_id(&self) -> String {
        DUMMY_METHOD_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "Dummy".to_owned()
    }
}
