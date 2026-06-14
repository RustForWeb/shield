use dioxus::prelude::*;

use crate::api::user;

#[component]
pub fn Home() -> Element {
    let user = use_loader(user)?;
    let user = user.read();

    rsx! {
        h1 { "Shield Dioxus Axum Example" }

        if let Some(user) = user.as_ref() {
            p {
                b {
                    "Name: "
                }
                span {
                    {user.name.as_deref().unwrap_or("-")}
                }
            }

            p {
                b {
                    "Email addresses:"
                }
                ul {
                    for email_address in &user.email_addresses {
                        li {
                            "{email_address}"
                        }
                    }
                }
            }

            a {
                href: "/auth/sign-out",
                "Sign out"
            }
        } else {
            a {
                href: "/auth/sign-in",
                "Sign in"
            }
        }
    }
}
