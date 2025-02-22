use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn home() -> String {
    let template = &ENV.get_template("home.html");
    let context = PageContext {
        data: extract_components!(&get_data!({ slug: "about" }), "TextContent"),
    };
    template.render(context!(page => context)).unwrap()
}
