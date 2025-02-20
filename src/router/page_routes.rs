use crate::pages::{
    about,
    blog::{article, blog},
    home,
};
use axum::{extract::Path, response::Html, routing::get, Router};

pub(crate) fn page_routes(router: Router) -> Router {
    router
        .route("/", get(Html(home())))
        .route("/about", get(Html(about())))
        .route("/blog", get(Html(blog())))
        .route(
            "/blog/{slug}",
            get(|params: Path<String>| async move { Html(article(params)) }),
        )
}
