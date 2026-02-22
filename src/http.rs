use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde_json::Value;
use std::{env, sync::LazyLock};

pub static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .max_idle_connections_per_host(10)
            .max_idle_age(std::time::Duration::from_secs(90))
            .build(),
    )
});

pub static ST_TOKEN: LazyLock<String> =
    LazyLock::new(|| env::var("ST_TOKEN").expect("ST_TOKEN not set"));
pub static ST_BASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ST_BASE_URL").expect("ST_BASE_URL not set"));
pub static AP_DATA: LazyLock<String> =
    LazyLock::new(|| env::var("AP_DATA").expect("AP_DATA not set"));

pub fn story(slug: &str) -> Option<Value> {
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

pub fn stories(prefix: &str) -> Vec<Value> {
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

pub fn text(v: &Value) -> String {
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

pub fn jstr<'a>(v: &'a Value, ptr: &str) -> &'a str {
    v.pointer(ptr).and_then(|v| v.as_str()).unwrap_or("")
}

pub fn jhtml(v: &Value, ptr: &str) -> String {
    htmd::convert(jstr(v, ptr)).unwrap_or_default()
}

pub fn blog_articles() -> impl Iterator<Item = Value> {
    stories("blog")
        .into_iter()
        .filter(|a| jstr(a, "/full_slug") != "blog/")
}

fn base_url() -> String {
    env::var("AP_BASE_URL")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string()
}

pub fn llms_content() -> Option<String> {
    let base = base_url();
    let mut body = format!("# Aprograma\n\n{}\n\n## Blog\n\n", text(&story("home")?));
    for a in blog_articles() {
        let (slug, title) = (jstr(&a, "/full_slug"), jstr(&a, "/content/title"));
        if !slug.is_empty() && !title.is_empty() {
            body.push_str(&format!("- [{title}]({base}/{slug}.md)\n"));
        }
    }
    Some(body)
}

pub fn home_md_content() -> Option<String> {
    Some(text(&story("home")?))
}

pub fn blog_md_content() -> String {
    let base = base_url();
    let items = blog_articles()
        .filter_map(|a| {
            let (t, s) = (jstr(&a, "/content/title"), jstr(&a, "/full_slug"));
            (!t.is_empty() && !s.is_empty()).then(|| format!("- [{t}]({base}/{s}.md)\n"))
        })
        .collect::<String>();
    format!("# Blog\n\n{items}")
}

pub fn article_md_content(slug: &str) -> Option<String> {
    let st = story(&format!("blog/{slug}"))?;
    Some(format!(
        "# {}\n\n## {}\n\n{}",
        jstr(&st, "/content/title"),
        jstr(&st, "/content/intro"),
        jhtml(&st, "/content/long_text")
    ))
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

pub fn xml_content(path: &str) -> Option<String> {
    AGENT
        .get(&format!("{}/{}", *AP_DATA, path.trim_start_matches('/')))
        .call()
        .ok()?
        .into_body()
        .read_to_string()
        .ok()
}
