use dioxus::prelude::*;

use crate::routes::Action;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum ShieldRouter {
    #[route("/:action_id")]
    Action { action_id: String },
}
