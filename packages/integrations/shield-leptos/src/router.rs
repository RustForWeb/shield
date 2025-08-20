use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};

use crate::routes::Action;

#[component(transparent)]
pub fn ShieldRouter() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("/") view=Action />
        <Route path=path!("/:action_id") view=Action />
    }
    .into_inner()
}
