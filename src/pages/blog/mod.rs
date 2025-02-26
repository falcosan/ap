export!(article);

use crate::environment::ENV;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn blog() -> String {
    let context = PageContext {
        data: get_data!({ starts_with: "blog" }),
    };
    ENV.render_template("blog.html", context)
}
