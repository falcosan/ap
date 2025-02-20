use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn about() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("about.html").unwrap();
    let context = PageContext {
        data: get_data!("about"),
    };
    template.render(context!(page => context)).unwrap()
}
