use crate::environment::ENV;
use axum::Json;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Serialize)]
struct PageContext {
    counter: i32,
    title: String,
}

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn read_counter() -> i32 {
    COUNTER.load(Ordering::SeqCst)
}

pub async fn increase_counter() -> Json<i32> {
    let _ = COUNTER.fetch_add(1, Ordering::SeqCst);
    Json(read_counter())
}

pub async fn decrease_counter() -> Json<i32> {
    let _ = COUNTER.fetch_sub(1, Ordering::SeqCst);
    Json(read_counter())
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        title: "Home".into(),
        counter: read_counter(),
    };

    template.render(context!(page => context)).unwrap()
}
