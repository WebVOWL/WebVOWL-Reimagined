#[cfg(feature = "wasm")]
pub use grapher::web::init_render;

// Expose an async initThreadPool function in the final generated JavaScript.
// You'll need to invoke it right after instantiating your module on the main
// thread in order to prepare the threadpool before calling into actual library functions.
#[cfg(feature = "wasm")]
pub use wasm_bindgen_rayon::init_thread_pool;

pub mod app;
pub mod components;
pub mod hydration_scripts;
pub mod pages;
pub mod network;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    use log::info;
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing logge");
    leptos::mount::hydrate_body(App);
    info!("Hydration complete");
}
