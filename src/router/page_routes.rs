use crate::pages::{
    blog::{article, blog},
    home,
};
use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode},
    response::{Html, Response},
    routing::get,
    Router,
};
use std::{env, sync::LazyLock};

static XML_API_BASE: LazyLock<String> =
    LazyLock::new(|| env::var("XML_API").expect("Missing XML_API environment variable"));

static CONTENT_TYPE_XML: LazyLock<HeaderValue> =
    LazyLock::new(|| HeaderValue::from_static("application/xml"));

async fn fetch_xml(path: &str) -> Result<String, StatusCode> {
    let url = format!("{}/{}", *XML_API_BASE, path);

    match reqwest::get(&url).await {
        Ok(response) => match response.text().await {
            Ok(content) => Ok(content),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn xml_handler(path: &str) -> Result<Response<String>, StatusCode> {
    let content = fetch_xml(path).await?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, CONTENT_TYPE_XML.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn page_routes(router: Router) -> Router {
    router
        .route("/", get(|| async move { Html(home()) }))
        .route("/blog", get(|| async move { Html(blog()) }))
        .route(
            "/blog/{slug}",
            get(|slug: Path<String>| async move { Html(article(slug)) }),
        )
        .route(
            "/rss.xml",
            get(|| async move { xml_handler("rss.xml").await }),
        )
        .route(
            "/sitemap.xml",
            get(|| async move { xml_handler("sitemap.xml").await }),
        )
}
