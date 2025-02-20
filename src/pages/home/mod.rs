use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        data: get_data!("home"),
    };
    template.render(context!(page => context)).unwrap()
}
