use crate::http::{
    article_md_content, blog_md_content, home_md_content, html, llms_content, md, wants_markdown,
    xml_content,
};
use crate::pages::{
    blog::{article, blog},
    home,
};
use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode, Uri},
    response::Response,
    routing::get,
    Router,
};

async fn llms_handler() -> Result<Response<String>, StatusCode> {
    md(llms_content().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?)
}

async fn home_handler(uri: Uri, headers: HeaderMap) -> Result<Response<String>, StatusCode> {
    if wants_markdown(&headers) {
        return md(home_md_content().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?);
    }
    html(home(uri.path()))
}

async fn blog_handler(uri: Uri, headers: HeaderMap) -> Result<Response<String>, StatusCode> {
    if wants_markdown(&headers) {
        return md(blog_md_content());
    }
    html(blog(uri.path()))
}

async fn xml_handler(uri: Uri) -> Result<Response<String>, StatusCode> {
    let body = xml_content(uri.path()).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Response::builder()
        .header(CONTENT_TYPE, "application/xml")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn home_md() -> Result<Response<String>, StatusCode> {
    md(home_md_content().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?)
}

async fn blog_md() -> Result<Response<String>, StatusCode> {
    md(blog_md_content())
}

async fn article_handler(
    uri: Uri,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Response<String>, StatusCode> {
    let (bare, as_md) = match slug.strip_suffix(".md") {
        Some(s) => (s, true),
        None => (slug.as_str(), wants_markdown(&headers)),
    };
    if as_md {
        return md(article_md_content(bare).ok_or(StatusCode::NOT_FOUND)?);
    }
    html(article(uri.path(), &slug))
}

pub fn page_routes() -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/.md", get(home_md))
        .route("/blog", get(blog_handler))
        .route("/blog.md", get(blog_md))
        .route("/blog/{slug}", get(article_handler))
        .route("/rss.xml", get(xml_handler))
        .route("/llms.txt", get(llms_handler))
        .route("/sitemap.xml", get(xml_handler))
}
