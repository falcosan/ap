use crate::environment::ENV;
use serde::Serialize;
use axum::extract::Path;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
    current_path: String,
}

pub fn article(current_path: &str, Path(slug): Path<String>) -> String {
    let context = PageContext {
        data: get_data!({ slug: format!("blog/{}", slug) }),
        current_path: current_path.to_string(),
    };
    ENV.render_template("article.html", context)
}
