#[derive(Clone, Debug)]
pub enum Response {
    // TODO: Remove temporary default variant.
    Default,
    Redirect(String),
}
