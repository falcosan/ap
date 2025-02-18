use minijinja::{context, Environment};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn app() {
    let mut env = Environment::new();

    env.add_template("hello.txt", "Hello {{ name }}!").unwrap();

    let template = env.get_template("hello.txt").unwrap();
    let ea = template.render(context!(name => "DAN")).unwrap();
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id("root") {
                element.set_inner_html(&ea);
            }
        }
    }
}
