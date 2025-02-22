export!(article);

use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
    articles: T,
}

pub fn blog() -> String {
    let template = &ENV.get_template("blog.html");
    let context = PageContext {
        data: get_data!({ slug: "blog" }),
        articles: get_data!({ starts_with: "blog" }),
    };
    template.render(context!(page => context)).unwrap()
}
