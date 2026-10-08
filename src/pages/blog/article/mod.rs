use crate::environment::render;
use crate::http::story;
use axum::http::StatusCode;

pub async fn article(current_path: &str, slug: &str) -> Result<String, StatusCode> {
    render("article.html", current_path, story(&["blog", slug]).await?)
}
