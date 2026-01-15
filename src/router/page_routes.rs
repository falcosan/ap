use crate::http::{ AGENT, ST_TOKEN, ST_BASE_URL };
use std::{ env, sync::LazyLock };
use serde_json::Value;
use axum::{
    Router,
    routing::get,
    extract::Path,
    response::{ Html, IntoResponse, Response },
    http::{ header::CONTENT_TYPE, StatusCode, Uri },
};
use crate::pages::{ blog::{ article, blog }, home };

static AP_DATA: LazyLock<String> = LazyLock::new(|| env::var("AP_DATA").expect("AP_DATA not set"));

fn extract_text(v: &Value) -> String {
    match v {
        Value::Object(obj) => {
            if
                obj
                    .get("component")
                    .map(|c| c == "TextContent")
                    .unwrap_or(false)
            {
                if let Some(t) = obj.get("text").and_then(|t| t.as_str()) {
                    return t.to_string();
                }
            }
            obj.values()
                .map(extract_text)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("\n\n")
        }
        Value::Array(arr) =>
            arr
                .iter()
                .map(extract_text)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("\n\n"),
        _ => String::new(),
    }
}

fn get_story(slug: &str) -> Option<Value> {
    AGENT.get(&format!("{}/{}?token={}", *ST_BASE_URL, slug, *ST_TOKEN))
        .call()
        .ok()?
        .into_body()
        .read_json::<Value>()
        .ok()?
        .get("story")
        .cloned()
}

fn get_stories(prefix: &str) -> Value {
    AGENT.get(&format!("{}?starts_with={}&token={}", *ST_BASE_URL, prefix, *ST_TOKEN))
        .call()
        .ok()
        .and_then(|r| r.into_body().read_json::<Value>().ok())
        .and_then(|j| j.get("stories").cloned())
        .unwrap_or(Value::Array(vec![]))
}

async fn llms_handler() -> Result<Response<String>, StatusCode> {
    let blog = get_stories("blog");
    let home = get_story("home").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let base = env::var("AP_BASE_URL").unwrap_or_default().trim_end_matches('/').to_string();
    let mut body = format!("# Aprograma\n\n{}\n\n## Blog\n\n", extract_text(&home));

    if let Value::Array(arr) = &blog {
        for a in arr
            .iter()
            .filter(|a| a.get("full_slug").and_then(|s| s.as_str()) != Some("blog/")) {
            if
                let (Some(slug), Some(title)) = (
                    a.get("full_slug").and_then(|s| s.as_str()),
                    a.pointer("/content/title").and_then(|s| s.as_str()),
                )
            {
                body.push_str(&format!("- [{title}]({base}/{slug})\n"));
            }
        }
    }

    Response::builder()
        .header(CONTENT_TYPE, "text/markdown; charset=utf-8")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn xml_handler(uri: Uri) -> Result<Response<String>, StatusCode> {
    let body = AGENT.get(&format!("{}/{}", *AP_DATA, uri.path().trim_start_matches('/')))
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_body()
        .read_to_string()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Response::builder()
        .header(CONTENT_TYPE, "application/xml")
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

pub fn page_routes() -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/blog", get(blog_handler))
        .route("/blog/{slug}", get(article_handler))
        .route("/rss.xml", get(xml_handler))
        .route("/llms.txt", get(llms_handler))
        .route("/sitemap.xml", get(xml_handler))
}
