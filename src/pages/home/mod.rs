use crate::environment::ENV;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn home() -> String {
    let context = PageContext {
        data: extract_components!(&get_data!({ slug: "home" }), "TextContent"),
    };
    ENV.render_template("home.html", context)
}
