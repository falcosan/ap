use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext {
    title: String,
}

pub fn fallback() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("fallback.html").unwrap();
    let context = PageContext {
        title: "404".into(),
    };

    template.render(context!(page => context)).unwrap()
}
