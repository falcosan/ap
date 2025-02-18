use minijinja::{context, Environment};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Serialize)]
struct Page {
    content: String,
}

#[wasm_bindgen]
pub fn app() {
    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("layout/default.html"))
        .unwrap();
    env.add_template("home.html", include_str!("pages/home.html"))
        .unwrap();

    let template = env.get_template("home.html").unwrap();
    let page = Page {
        content: "Lorem Ipsum".into(),
    };
    let render = template.render(context!(page)).unwrap();

    if let Some(root) = window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.get_element_by_id("root"))
    {
        root.set_inner_html(&render);
    }
}
