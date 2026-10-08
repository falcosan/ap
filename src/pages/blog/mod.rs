pub mod article;

use crate::environment::render;
use crate::http::blog_articles;
use axum::http::StatusCode;

pub async fn blog(current_path: &str) -> Result<String, StatusCode> {
    render("blog.html", current_path, blog_articles().await?)
}
