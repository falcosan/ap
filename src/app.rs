use minijinja::{context, Environment};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Serialize)]
struct Page {
    content: String,
}

fn layout(env: &mut Environment) {
    env.add_template("layout.html", include_str!("layout/default.jinja"))
        .unwrap();
}

fn home(env: &mut Environment) -> String {
    env.add_template("home.html", include_str!("pages/home.jinja"))
        .unwrap();

    let template = env.get_template("home.html").unwrap();
    let page = Page {
        content: "Lorem Ipsum".into(),
    };
    template.render(context!(page)).unwrap()
}

#[wasm_bindgen]
pub fn app() {
    let mut env = Environment::new();

    layout(&mut env);
    let home = home(&mut env);

    if let Some(root) = window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.get_element_by_id("root"))
    {
        root.set_inner_html(&home);
    }
}
