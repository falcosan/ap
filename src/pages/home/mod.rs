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

fn read_counter() -> String {
    COUNTER.load(Ordering::SeqCst).to_string()
}

pub async fn increase_counter() -> String {
    let _ = COUNTER.fetch_add(1, Ordering::SeqCst);
    read_counter()
}

pub async fn decrease_counter() -> String {
    let _ = COUNTER.fetch_sub(1, Ordering::SeqCst);
    read_counter()
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        counter: read_counter(),
        content: "Home".into(),
    };

    template.render(context!(page => context)).unwrap()
}
