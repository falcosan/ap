mod environment;

use environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};
use wasm_bindgen::prelude::*;
use web_sys::window;

fn home() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
        counter: String,
    }

    static COUNTER: AtomicI32 = AtomicI32::new(0);

    let mut env = ENV.lock().unwrap();

    env.add_function("increment", |val: i32| {
        COUNTER.fetch_add(val, Ordering::SeqCst);
    });

    env.add_function("decrement", |val: i32| {
        COUNTER.fetch_sub(val, Ordering::SeqCst);
    });

    let template = env.get_template("home.html").unwrap();

    let page = Props {
        content: "Lorem Ipsum".into(),
        counter: COUNTER.load(Ordering::SeqCst).to_string(),
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
