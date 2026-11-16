pub mod app;
mod home;

#[expect(unused_imports)]
use console_error_panic_hook::set_once as _;
#[expect(unused_imports)]
use wasm_tracing::set_as_global_default as _;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    leptos::mount::hydrate_body(App);
}
