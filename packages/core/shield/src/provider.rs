pub trait Provider: Send + Sync {
    fn method_id(&self) -> String;

    fn id(&self) -> Option<String>;

    fn name(&self) -> String;
}
