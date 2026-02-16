use crate::http::{AGENT, ST_BASE_URL, ST_TOKEN};
use crate::pages::{
    blog::{article, blog},
    home,
};
use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde_json::Value;
use std::{env, sync::LazyLock};

static AP_DATA: LazyLock<String> = LazyLock::new(|| env::var("AP_DATA").expect("AP_DATA not set"));

fn res(ct: &str, body: String) -> Result<Response<String>, StatusCode> {
    Response::builder()
        .header(CONTENT_TYPE, ct)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn md(body: String) -> Result<Response<String>, StatusCode> {
    res("text/markdown; charset=utf-8", body)
}
fn html(body: String) -> Result<Response<String>, StatusCode> {
    res("text/html; charset=utf-8", body)
}

fn story(slug: &str) -> Option<Value> {
    AGENT
        .get(&format!("{}/{}?token={}", *ST_BASE_URL, slug, *ST_TOKEN))
        .call()
        .ok()?
        .into_body()
        .read_json::<Value>()
        .ok()?
        .get("story")
        .cloned()
}

fn stories(prefix: &str) -> Vec<Value> {
    AGENT
        .get(&format!(
            "{}?starts_with={}&token={}",
            *ST_BASE_URL, prefix, *ST_TOKEN
        ))
        .call()
        .ok()
        .and_then(|r| r.into_body().read_json::<Value>().ok())
        .and_then(|j| j.get("stories").cloned())
        .and_then(|v| {
            if let Value::Array(a) = v {
                Some(a)
            } else {
                None
            }
        })
        .unwrap_or_default()
}

fn text(v: &Value) -> String {
    match v {
        Value::Object(o)
            if o.get("component")
                .map(|c| c == "TextContent")
                .unwrap_or(false) =>
        {
            o.get("text").and_then(|t| t.as_str()).unwrap_or("").into()
        }
        Value::Object(o) => o
            .values()
            .map(text)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n"),
        Value::Array(a) => a
            .iter()
            .map(text)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n"),
        _ => String::new(),
    }
}

fn jstr<'a>(v: &'a Value, ptr: &str) -> &'a str {
    v.pointer(ptr).and_then(|v| v.as_str()).unwrap_or("")
}

fn jhtml(v: &Value, ptr: &str) -> String {
    htmd::convert(jstr(v, ptr)).unwrap_or_default()
}

fn blog_articles() -> impl Iterator<Item = Value> {
    stories("blog")
        .into_iter()
        .filter(|a| jstr(a, "/full_slug") != "blog/")
}

async fn llms_handler() -> Result<Response<String>, StatusCode> {
    let base = env::var("AP_BASE_URL")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string();
    let mut body = format!(
        "# Aprograma\n\n{}\n\n## Blog\n\n",
        text(&story("home").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?)
    );
    for a in blog_articles() {
        let (slug, title) = (jstr(&a, "/full_slug"), jstr(&a, "/content/title"));
        if !slug.is_empty() && !title.is_empty() {
            body.push_str(&format!("- [{title}]({base}/{slug}.md)\n"));
        }
    }
    md(body)
}

async fn home_handler(uri: Uri) -> impl IntoResponse {
    Html(home(uri.path()))
}

async fn blog_handler(uri: Uri) -> impl IntoResponse {
    Html(blog(uri.path()))
}

async fn xml_handler(uri: Uri) -> Result<Response<String>, StatusCode> {
    let body = AGENT
        .get(&format!(
            "{}/{}",
            *AP_DATA,
            uri.path().trim_start_matches('/')
        ))
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_body()
        .read_to_string()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    res("application/xml", body)
}

async fn home_md() -> Result<Response<String>, StatusCode> {
    md(text(
        &story("home").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

async fn blog_md() -> Result<Response<String>, StatusCode> {
    let base = env::var("AP_BASE_URL")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string();
    let items = blog_articles()
        .filter_map(|a| {
            let (t, s) = (jstr(&a, "/content/title"), jstr(&a, "/full_slug"));
            (!t.is_empty() && !s.is_empty()).then(|| format!("- [{t}]({base}/{s}.md)\n"))
        })
        .collect::<String>();
    md(format!("# Blog\n\n{items}"))
}

async fn article_handler(
    uri: Uri,
    Path(slug): Path<String>,
) -> Result<Response<String>, StatusCode> {
    if let Some(s) = slug.strip_suffix(".md") {
        let st = story(&format!("blog/{s}")).ok_or(StatusCode::NOT_FOUND)?;
        return md(format!(
            "# {}\n\n## {}\n\n{}",
            jstr(&st, "/content/title"),
            jstr(&st, "/content/intro"),
            jhtml(&st, "/content/long_text")
        ));
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
