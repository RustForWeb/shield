use std::{ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct LeptosRedirect(Arc<dyn Fn(&str) + Send + Sync>);

impl Deref for LeptosRedirect {
    type Target = Arc<dyn Fn(&str) + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(&str) + Send + Sync + 'static> From<F> for LeptosRedirect {
    fn from(value: F) -> Self {
        LeptosRedirect(Arc::new(value))
    }
}
