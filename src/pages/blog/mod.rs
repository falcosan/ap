use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn blog() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("blog.html").unwrap();
    let context = PageContext {
        data: get_data!({ slug: "blog" }),
    };
    template.render(context!(page => context)).unwrap()
}
