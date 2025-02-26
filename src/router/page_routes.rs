use crate::pages::{
    blog::{article, blog},
    home,
};
use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{env, error::Error, sync::LazyLock};

static DATA_AP_BASE: LazyLock<String> =
    LazyLock::new(|| env::var("DATA_AP").expect("Missing DATA_AP environment variable"));

async fn fetch_xml(path: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/{}", *DATA_AP_BASE, path);
    let response = ureq::get(&url).call()?.body_mut().read_to_string()?;

    Ok(response)
}

async fn xml_handler(path: &str) -> Result<Response<String>, StatusCode> {
    let content = fetch_xml(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    static CONTENT_TYPE_XML: LazyLock<HeaderValue> =
        LazyLock::new(|| "application/xml".parse().unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, CONTENT_TYPE_XML.clone());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(content)
        .unwrap())
}

async fn home_handler() -> impl IntoResponse {
    Html(home())
}

async fn blog_handler() -> impl IntoResponse {
    Html(blog())
}

async fn article_handler(slug: Path<String>) -> impl IntoResponse {
    Html(article(slug))
}

pub fn page_routes(router: Router) -> Router {
    router
        .route("/", get(home_handler))
        .route("/blog", get(blog_handler))
        .route("/blog/{slug}", get(article_handler))
        .route("/rss.xml", get(|| xml_handler("rss.xml")))
        .route("/sitemap.xml", get(|| xml_handler("sitemap.xml")))
}
