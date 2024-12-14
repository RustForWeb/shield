use std::any::Any;

pub trait Storage: Any {
    fn id(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
pub(crate) mod tests {
    use std::any::Any;

    use crate::provider::tests::TestProviderStorage;

    use super::Storage;

    pub const TEST_STORAGE_ID: &str = "test";

    #[derive(Default)]
    pub struct TestStorage {}

    impl Storage for TestStorage {
        fn id(&self) -> &'static str {
            TEST_STORAGE_ID
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl TestProviderStorage for TestStorage {}
}
