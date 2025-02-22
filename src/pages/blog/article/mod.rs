use crate::environment::ENV;
use axum::extract::Path;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn article(Path(slug): Path<String>) -> String {
    let template = &ENV.get_template("article.html");
    let context = PageContext {
        data: get_data!({ slug: format!("blog/{}", slug) }),
    };
    template.render(context!(page => context)).unwrap()
}
