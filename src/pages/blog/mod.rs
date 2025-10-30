export!(article);

use crate::environment::ENV;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
    current_path: String,
}

pub fn blog(current_path: &str) -> String {
    let context = PageContext {
        data: get_data!({ starts_with: "blog" }),
        current_path: current_path.to_string(),
    };
    ENV.render_template("blog.html", context)
}
