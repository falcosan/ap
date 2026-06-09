use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde_json::Value;
use std::fmt::Write;
use std::time::Duration;
use std::{env, sync::LazyLock};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to build HTTP client")
});

pub static ST_TOKEN: LazyLock<String> =
    LazyLock::new(|| env::var("ST_TOKEN").expect("ST_TOKEN not set"));
pub static ST_BASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ST_BASE_URL").expect("ST_BASE_URL not set"));
pub static AP_DATA: LazyLock<String> =
    LazyLock::new(|| env::var("AP_DATA").expect("AP_DATA not set"));
static AP_BASE_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("AP_BASE_URL")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string()
});

pub async fn fetch_json(url: String) -> Option<Value> {
    CLIENT
        .get(url)
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json()
        .await
        .ok()
}

pub async fn story(slug: &str) -> Option<Value> {
    fetch_json(format!("{}/{}?token={}", *ST_BASE_URL, slug, *ST_TOKEN))
        .await?
        .as_object_mut()?
        .remove("story")
}

pub async fn stories(prefix: &str) -> Vec<Value> {
    let stories = fetch_json(format!(
        "{}?starts_with={}&token={}",
        *ST_BASE_URL, prefix, *ST_TOKEN
    ))
    .await
    .and_then(|mut json| json.as_object_mut()?.remove("stories"));

    match stories {
        Some(Value::Array(a)) => a,
        _ => Vec::new(),
    }
}

pub fn text(v: &Value) -> String {
    let mut out = String::new();
    collect_text(v, &mut out);
    out
}

fn collect_text(v: &Value, out: &mut String) {
    match v {
        Value::Object(o)
            if o.get("component")
                .map(|c| c == "TextContent")
                .unwrap_or(false) =>
        {
            if let Some(t) = o.get("text").and_then(|t| t.as_str()) {
                if !t.is_empty() {
                    if !out.is_empty() {
                        out.push_str("\n\n");
                    }
                    out.push_str(t);
                }
            }
        }
        Value::Object(o) => {
            for v in o.values() {
                collect_text(v, out);
            }
        }
        Value::Array(a) => {
            for v in a {
                collect_text(v, out);
            }
        }
        _ => {}
    }
}

pub fn jstr<'a>(v: &'a Value, ptr: &str) -> &'a str {
    v.pointer(ptr).and_then(|v| v.as_str()).unwrap_or("")
}

pub fn jhtml(v: &Value, ptr: &str) -> String {
    htmd::convert(jstr(v, ptr)).unwrap_or_default()
}

pub fn md(body: String) -> Result<Response<String>, StatusCode> {
    Response::builder()
        .header(CONTENT_TYPE, "text/markdown; charset=utf-8")
        .header("Vary", "Accept")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn html(body: String) -> Result<Response<String>, StatusCode> {
    Response::builder()
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .header("Vary", "Accept")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn wants_markdown(headers: &HeaderMap) -> bool {
    headers
        .get(ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.to_ascii_lowercase().contains("text/markdown"))
}

pub async fn blog_articles() -> Vec<Value> {
    let mut articles = stories("blog").await;
    articles.retain(|a| jstr(a, "/full_slug") != "blog/");
    articles
}

pub async fn llms_content() -> Option<String> {
    let (home, articles) = tokio::join!(story("home"), blog_articles());
    let base = AP_BASE_URL.as_str();
    let mut body = format!("# Aprograma\n\n{}\n\n## Blog\n\n", text(&home?));
    for a in &articles {
        let (slug, title) = (jstr(a, "/full_slug"), jstr(a, "/content/title"));
        if !slug.is_empty() && !title.is_empty() {
            let _ = writeln!(body, "- [{title}]({base}/{slug}.md)");
        }
    }
    Some(body)
}

pub async fn home_md_content() -> Option<String> {
    Some(text(&story("home").await?))
}

pub async fn blog_md_content() -> String {
    let base = AP_BASE_URL.as_str();
    let mut body = String::from("# Blog\n\n");
    for a in blog_articles().await {
        let (t, s) = (jstr(&a, "/content/title"), jstr(&a, "/full_slug"));
        if !t.is_empty() && !s.is_empty() {
            let _ = writeln!(body, "- [{t}]({base}/{s}.md)");
        }
    }
    body
}

pub async fn article_md_content(slug: &str) -> Option<String> {
    let st = story(&format!("blog/{slug}")).await?;
    Some(format!(
        "# {}\n\n## {}\n\n{}",
        jstr(&st, "/content/title"),
        jstr(&st, "/content/intro"),
        jhtml(&st, "/content/long_text")
    ))
}

pub async fn xml_content(path: &str) -> Option<String> {
    CLIENT
        .get(format!("{}/{}", *AP_DATA, path.trim_start_matches('/')))
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .text()
        .await
        .ok()
}
