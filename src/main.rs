mod templates;

use minijinja::context;
use serde::Serialize;
use templates::TEMPLATES;
use wasm_bindgen::prelude::*;
use web_sys::window;

fn home() -> String {
    #[derive(Serialize)]
    struct Page {
        content: String,
    }

    let template = TEMPLATES.get_template("home.html").unwrap();
    let page = Page {
        content: "Lorem Ipsum".into(),
    };
    template.render(context!(page)).unwrap()
}

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
