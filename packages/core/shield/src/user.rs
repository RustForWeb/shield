pub trait User: Send + Sync {
    fn id(&self) -> String;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::User;

    #[derive(Clone, Debug)]
    pub struct TestUser {
        id: String,
    }

    impl User for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }
    }
}
