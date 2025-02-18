#[macro_use]
mod macros;
mod environment;

mod pages {
    export!(home);
}

use pages::home;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn app() {
    let home = home();

    if let Some(root) = window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.get_element_by_id("root"))
    {
        root.set_inner_html(&home);
    }
}
