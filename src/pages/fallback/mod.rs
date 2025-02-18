use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn fallback() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
    }

    let mut env = ENV.lock().unwrap();

    static COUNTER: AtomicI32 = AtomicI32::new(0);

    env.add_function("increment", |val: i32| {
        COUNTER.fetch_add(val, Ordering::SeqCst)
    });

    let template = env.get_template("fallback.html").unwrap();

    let page = Props {
        content: "404".into(),
    };

    template.render(context!(page)).unwrap()
}
