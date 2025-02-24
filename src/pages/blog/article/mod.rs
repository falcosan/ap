use crate::environment::ENV;
use axum::extract::Path;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn article(Path(slug): Path<String>) -> String {
    let context = PageContext {
        data: get_data!({ slug: format!("blog/{}", slug) }),
    };
    ENV.render_template("article.html", context)
}
