use dioxus::prelude::*;

use crate::routes::Action;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum ShieldRouter {
    #[route("?:..query", Action)]
    ActionIndex { query: String },
    #[route("/:action_id?:..query")]
    Action { action_id: String, query: String },
}
