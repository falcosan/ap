use crate::environment::{ PageContext, ENV };
use axum::extract::Path;

pub fn article(current_path: &str, Path(slug): Path<String>) -> String {
    ENV.render_template("article.html", PageContext {
        data: get_data!({ slug: format!("blog/{}", slug) }),
        current_path: current_path.to_string(),
    })
}
