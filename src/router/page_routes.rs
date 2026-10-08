use crate::http::{
    article_md_content, blog_md_content, home_md_content, html, llms_content, md, not_found,
    wants_markdown, xml_content,
};
use crate::pages::{
    blog::{article::article, blog},
    home::home,
};
use axum::{
    Router,
    extract::Path,
    http::{HeaderMap, StatusCode, Uri, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
    routing::get,
};

async fn llms_handler() -> Result<Response, StatusCode> {
    Ok(md(llms_content().await?))
}

async fn home_handler(uri: Uri, headers: HeaderMap) -> Result<Response, StatusCode> {
    if wants_markdown(&headers) {
        return home_md().await;
    }
    Ok(html(home(uri.path()).await?))
}

async fn blog_handler(uri: Uri, headers: HeaderMap) -> Result<Response, StatusCode> {
    if wants_markdown(&headers) {
        return blog_md().await;
    }
    Ok(html(blog(uri.path()).await?))
}

async fn xml_handler(uri: Uri) -> Result<Response, StatusCode> {
    let body = xml_content(uri.path()).await?;
    Ok(([(CONTENT_TYPE, "application/xml")], body).into_response())
}

async fn home_md() -> Result<Response, StatusCode> {
    Ok(md(home_md_content().await?))
}

async fn blog_md() -> Result<Response, StatusCode> {
    Ok(md(blog_md_content().await?))
}

async fn article_handler(
    uri: Uri,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let (bare, as_md) = slug
        .strip_suffix(".md")
        .map_or_else(|| (slug.as_str(), wants_markdown(&headers)), |s| (s, true));
    if as_md {
        return Ok(md(article_md_content(bare).await?));
    }
    match article(uri.path(), &slug).await {
        Err(StatusCode::NOT_FOUND) => not_found().await,
        page => Ok(html(page?)),
    }
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
