#[macro_use]
mod macros;
mod environment;
mod router;
mod pages {
    export!(home);
    export!(about);
    export!(blog);
    export!(fallback);
}

use pages::{about, blog, fallback, home};
use router::{get_window_location, render_page};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn app() {
    match get_window_location() {
        Some(location) => {
            let path = location.pathname().unwrap_or_default();
            match path.as_str() {
                "/" => render_page(home),
                "/blog" => render_page(blog),
                "/about" => render_page(about),
                _ => render_page(fallback),
            }
        }
        None => console::error_1(&"Failed to get window location".into()),
    }
}
