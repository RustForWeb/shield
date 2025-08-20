use dioxus::{document::Stylesheet, prelude::*};
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
        Stylesheet {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/css/bootstrap.min.css",
            integrity: "sha384-LN+7fdVzj6u52u30Kp6M/trliBMCMKTyK833zpbD+pXdCLuTusPj697FH4R/5mcr",
            crossorigin: "anonymous"
        }

        main {
            Router::<Route> {}
        }
    }
}
