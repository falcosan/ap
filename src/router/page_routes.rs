use crate::pages::{about, blog, home};
use axum::{response::Html, routing::get, Router};

async fn home_handler() -> Html<String> {
    Html(home())
}

async fn about_handler() -> Html<String> {
    Html(about())
}

async fn blog_handler() -> Html<String> {
    Html(blog())
}

pub(crate) fn page_routes(router: Router) -> Router {
    router
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/blog", get(blog_handler))
}
