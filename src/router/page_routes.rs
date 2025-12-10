use crate::http::AGENT;
use std::{ env, sync::LazyLock };
use html_to_markdown_rs::convert;
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::{ Html, IntoResponse, Response },
    http::{ header::CONTENT_TYPE, StatusCode, Uri },
};
use crate::pages::{ blog::{ article, blog }, home };

static AP_DATA: LazyLock<String> = LazyLock::new(|| env::var("AP_DATA").expect("AP_DATA not set"));

fn fetch_xml(path: &str) -> Result<String, StatusCode> {
    AGENT.get(&format!("{}/{}", *AP_DATA, path))
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_body()
        .read_to_string()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn xml_handler(path: &str) -> Result<Response<String>, StatusCode> {
    let body = fetch_xml(path)?;
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/xml")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn md_handler(path: &str) -> Result<Response<String>, StatusCode> {
    let body = convert(&fetch_xml(path)?, None).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/markdown; charset=utf-8")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn home_handler(uri: Uri) -> impl IntoResponse {
    Html(home(uri.path()))
}

async fn blog_handler(uri: Uri) -> impl IntoResponse {
    Html(blog(uri.path()))
}

async fn article_handler(uri: Uri, Path(slug): Path<String>) -> impl IntoResponse {
    Html(article(uri.path(), slug.as_str()))
}

pub fn page_routes(router: Router) -> Router {
    router
        .route("/", get(home_handler))
        .route("/blog", get(blog_handler))
        .route("/blog/{slug}", get(article_handler))
        .route(
            "/rss.xml",
            get(|| xml_handler("rss.xml"))
        )
        .route(
            "/llms.txt",
            get(|| md_handler("llms.txt"))
        )
        .route(
            "/sitemap.xml",
            get(|| xml_handler("sitemap.xml"))
        )
}
