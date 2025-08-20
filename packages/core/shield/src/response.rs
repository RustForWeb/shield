use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    // TODO: Remove temporary default variant.
    Default,
    Redirect(String),
    RedirectToAction { action_id: String },
}
