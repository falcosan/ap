use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Serialize)]
struct PageContext {
    content: String,
    counter: String,
}

static COUNTER: AtomicI32 = AtomicI32::new(0);

pub async fn increment_counter() -> String {
    let res = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    res.to_string()
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        content: "Home".into(),
        counter: COUNTER.load(Ordering::SeqCst).to_string(),
    };

    template.render(context!(page => context)).unwrap()
}
