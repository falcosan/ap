use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext {
    content: String,
}

pub fn about() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("about.html").unwrap();
    let context = PageContext {
        content: "About".into(),
    };

    template.render(context!(page => context)).unwrap()
}
