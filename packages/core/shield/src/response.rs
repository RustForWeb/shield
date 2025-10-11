use serde::{Deserialize, Serialize};

use crate::SessionAction;

#[derive(Clone, Debug)]
pub struct Response {
    pub r#type: ResponseType,
    pub session_actions: Vec<SessionAction>,
}

impl Response {
    pub fn new(r#type: ResponseType) -> Self {
        Self {
            r#type,
            session_actions: vec![],
        }
    }

    pub fn session_action(mut self, session_action: SessionAction) -> Self {
        self.session_actions.push(session_action);
        self
    }

    pub fn session_actions(mut self, session_actions: &mut Vec<SessionAction>) -> Self {
        self.session_actions.append(session_actions);
        self
    }
}

// TODO: Rename to something more sensible.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ResponseType {
    // TODO: Remove temporary default variant.
    Default,
    Redirect(String),
    RedirectToAction { action_id: String },
}
