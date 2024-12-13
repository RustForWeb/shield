pub trait Provider {
    fn id(&self) -> &'static str;

    fn sign_in(&self);

    fn sign_out(&self);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Provider;

    pub const TEST_PROVIDER_ID: &str = "test";

    #[derive(Default)]
    pub struct TestProvider {
        id: Option<&'static str>,
    }

    impl TestProvider {
        pub fn with_id(mut self, id: &'static str) -> Self {
            self.id = Some(id);
            self
        }
    }

    impl Provider for TestProvider {
        fn id(&self) -> &'static str {
            self.id.unwrap_or(TEST_PROVIDER_ID)
        }

        fn sign_in(&self) {}

        fn sign_out(&self) {}
    }
}
