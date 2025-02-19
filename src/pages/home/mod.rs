use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn home() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
        counter: i32,
    }

    let mut env = ENV.lock().unwrap();
    static COUNTER: AtomicI32 = AtomicI32::new(0);

    env.add_function("increment", |val: i32| {
        COUNTER.fetch_add(val, Ordering::SeqCst)
    });

    let template = env.get_template("home.html").unwrap();

    let page = Props {
        content: "Home".into(),
        counter: COUNTER.load(Ordering::SeqCst),
    };

    template.render(context!(page)).unwrap()
}
