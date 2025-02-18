#[macro_use]
mod macros;
mod environment;
mod router;
mod pages {
    export!(home);
    export!(about);
    export!(blog);
}

use pages::{about, blog, home};
use router::{get_window_location, render_page};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn app() {
    match get_window_location() {
        Some(location) => {
            let path = location.pathname().unwrap_or_default();
            match path.as_str() {
                "/blog" => render_page(blog),
                "/about" => render_page(about),
                _ => render_page(home),
            }
        }
        None => console::error_1(&"Failed to get window location".into()),
    }
}
