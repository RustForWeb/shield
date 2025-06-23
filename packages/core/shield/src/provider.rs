pub trait Provider: Send + Sync {
    fn method_id(&self) -> String;

    fn id(&self) -> Option<String>;

    fn name(&self) -> String;
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;

    use crate::method::tests::TEST_METHOD_ID;

    use super::Provider;

    pub const TEST_PROVIDER_NAME: &str = "Test";

    #[derive(Default)]
    pub struct TestProvider {}

    #[async_trait]
    impl Provider for TestProvider {
        fn method_id(&self) -> String {
            TEST_METHOD_ID.to_owned()
        }

        fn id(&self) -> Option<String> {
            None
        }

        fn name(&self) -> String {
            TEST_PROVIDER_NAME.to_owned()
        }
    }
}
