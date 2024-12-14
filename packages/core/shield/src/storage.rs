pub trait Storage {
    fn id(&self) -> &'static str;
}

#[derive(Debug)]
pub enum StorageError {}

#[cfg(test)]
pub(crate) mod tests {
    use super::Storage;

    pub const TEST_STORAGE_ID: &str = "test";

    #[derive(Default)]
    pub struct TestStorage {}

    impl Storage for TestStorage {
        fn id(&self) -> &'static str {
            TEST_STORAGE_ID
        }
    }
}
