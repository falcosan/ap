use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext {
    content: String,
}

pub fn fallback() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("fallback.html").unwrap();
    let context = PageContext {
        content: "404".into(),
    };

    template.render(context!(page => context)).unwrap()
}
