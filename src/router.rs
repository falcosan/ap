use crate::pages::{about, blog, fallback, home};
use axum::{http::StatusCode, response::Html, routing::get, Router};

async fn home_handler() -> Html<String> {
    Html(home())
}

async fn about_handler() -> Html<String> {
    Html(about())
}

async fn blog_handler() -> Html<String> {
    Html(blog())
}

async fn fallback_handler() -> (StatusCode, Html<String>) {
    (StatusCode::NOT_FOUND, Html(fallback()))
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/blog", get(blog_handler))
        .fallback(fallback_handler)
}
