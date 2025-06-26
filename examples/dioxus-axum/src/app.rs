use dioxus::prelude::*;

use crate::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        main {
            Router::<Route> {}
        }
    }
}
