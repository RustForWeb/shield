use dioxus::prelude::*;
use shield::InputAddon;

#[derive(Clone, PartialEq, Props)]
pub struct FormInputAddonProps {
    addon: InputAddon,
    group: bool,
}

#[component]
pub fn FormInputAddon(props: FormInputAddonProps) -> Element {
    match props.addon {
        InputAddon::Image { alt, src } => rsx! {
            img {
                src,
                alt,
                width: 16,
                height: 16,
            }
        },
        InputAddon::Text { text } => {
            if props.group {
                rsx! {
                    span {
                        class: "input-group-text",
                        "{text}"
                    }
                }
            } else {
                rsx! {
                    span {
                        "{text}"
                    }
                }
            }
        }
    }
}
