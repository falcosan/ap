use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext {
    title: String,
}

pub fn blog() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("blog.html").unwrap();
    let context = PageContext {
        title: "Blog".into(),
    };

    template.render(context!(page => context)).unwrap()
}
