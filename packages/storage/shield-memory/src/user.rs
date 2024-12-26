#[derive(Clone, Debug)]
pub struct User {
    id: String,
}

impl shield::User for User {
    fn id(&self) -> String {
        self.id.clone()
    }
}
