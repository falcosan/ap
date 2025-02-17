use tera::Tera;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn app() {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id("root") {
                match Tera::new("templates/**/*.html") {
                    Ok(t) => {
                        let rendered = t.render("index.html", &tera::Context::new()).unwrap();
                        element.set_inner_html(&rendered)
                    }
                    Err(e) => {
                        println!("Parsing error(s): {}", e);
                        ::std::process::exit(1);
                    }
                };
            }
        }
    }
}
