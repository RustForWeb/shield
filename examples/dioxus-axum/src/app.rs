use dioxus::prelude::*;
use shield_dioxus::ShieldRouter;

use crate::home::Home;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[child("/auth")]
    Auth {
        child: ShieldRouter
    },
}

#[component]
pub fn App() -> Element {
    rsx! {
        main {
            Router::<Route> {}
        }
    }
}
