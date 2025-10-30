use crate::environment::ENV;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
    current_path: String,
}

pub fn home(current_path: &str) -> String {
    let context = PageContext {
        data: extract_components!(&get_data!({ slug: "home" }), "TextContent"),
        current_path: current_path.to_string(),
    };
    ENV.render_template("home.html", context)
}
