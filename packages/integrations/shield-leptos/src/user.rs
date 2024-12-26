use std::sync::Arc;

use shield::User;

// TODO: This doesn't work, but Leptos does not support generics in server functions.
pub struct LeptosUser(Arc<dyn User>);

impl User for LeptosUser {
    fn id(&self) -> String {
        self.0.id()
    }
}
