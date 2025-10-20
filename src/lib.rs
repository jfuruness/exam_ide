use leptos::*;

mod components;
mod python_runner;
mod storage;
mod url_codec;

use components::app::App;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
