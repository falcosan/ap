use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn home() -> String {
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
