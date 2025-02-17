use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn app() {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id("root") {
                element.set_inner_html("Hello from Rust!");
            }
        }
    }
}
