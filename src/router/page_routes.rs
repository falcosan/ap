use crate::pages::{
    blog::{article, blog},
    home,
};
use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderName},
    response::Html,
    routing::get,
    Router,
};
use std::{env, error::Error};

fn fetch_xml(url: &str) -> Result<String, Box<dyn Error>> {
    minreq::get(format!("{}{}", env::var("XML_API")?, url))
        .send()?
        .as_str()
        .map(String::from)
        .or_else(|_| Err("Failed to read response as string".into()))
}

fn xml_response(
    content: Result<String, Box<dyn Error>>,
) -> ([(HeaderName, &'static str); 1], String) {
    let content = content.unwrap_or_else(|e| format!("Error: {}", e));
    ([(CONTENT_TYPE, "application/xml")], content)
}
pub fn page_routes(router: Router) -> Router {
    router
        .route("/", get(|| async { Html(home()) }))
        .route("/blog", get(|| async { Html(blog()) }))
        .route(
            "/blog/{slug}",
            get(|params: Path<String>| async { Html(article(params)) }),
        )
        .route("/rss.xml", get(xml_response(fetch_xml("rss.xml"))))
        .route("/sitemap.xml", get(xml_response(fetch_xml("sitemap.xml"))))
}
