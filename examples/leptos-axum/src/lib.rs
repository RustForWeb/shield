pub mod app;
mod home;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::prelude::provide_context;
    use shield_bootstrap::BootstrapLeptosStyle;

    use crate::app::*;

    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    leptos::mount::hydrate_body(|| {
        provide_context(BootstrapLeptosStyle::default().context());

        App()
    });
}
