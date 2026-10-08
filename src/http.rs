use crate::pages::fallback::fallback;
use axum::http::header::{ACCEPT, CONTENT_TYPE, VARY};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use reqwest::Url;
use serde_json::Value;
use std::time::Duration;
use std::{env, sync::LazyLock};
use tracing::{error, warn};

const STORIES_PER_PAGE: usize = 100;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to build HTTP client")
});

static ST_TOKEN: LazyLock<String> =
    LazyLock::new(|| env::var("ST_TOKEN").expect("ST_TOKEN not set"));
static ST_BASE_URL: LazyLock<Url> = LazyLock::new(|| env_url("ST_BASE_URL"));
static AP_DATA: LazyLock<Url> = LazyLock::new(|| env_url("AP_DATA"));
static AP_BASE_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("AP_BASE_URL")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string()
});

pub fn load_config() {
    LazyLock::force(&ST_TOKEN);
    LazyLock::force(&ST_BASE_URL);
    LazyLock::force(&AP_DATA);
}

fn env_url(key: &str) -> Url {
    env::var(key)
        .unwrap_or_else(|_| panic!("{key} not set"))
        .parse()
        .unwrap_or_else(|_| panic!("{key} is not a valid URL"))
}

fn join(base: &Url, path: &[&str]) -> Url {
    let mut url = base.clone();
    url.path_segments_mut()
        .expect("Base URL cannot have path segments")
        .pop_if_empty()
        .extend(path);
    url
}

fn storyblok_url(path: &[&str], query: &[(&str, &str)]) -> Url {
    let mut url = join(&ST_BASE_URL, path);
    url.query_pairs_mut()
        .extend_pairs(query)
        .append_pair("token", &ST_TOKEN);
    url
}

fn upstream_error(error: reqwest::Error) -> StatusCode {
    if error.status() == Some(StatusCode::NOT_FOUND) {
        return StatusCode::NOT_FOUND;
    }
    let path = error.url().map(|url| url.path().to_owned());
    warn!(path, error = %error.without_url(), "Upstream request failed");
    StatusCode::BAD_GATEWAY
}

async fn fetch(url: Url) -> Result<reqwest::Response, StatusCode> {
    CLIENT
        .get(url)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(upstream_error)
}

async fn fetch_field(url: Url, field: &str) -> Result<Value, StatusCode> {
    fetch(url)
        .await?
        .json::<Value>()
        .await
        .map_err(upstream_error)?
        .get_mut(field)
        .map(Value::take)
        .ok_or(StatusCode::BAD_GATEWAY)
}

pub async fn story(path: &[&str]) -> Result<Value, StatusCode> {
    if path
        .iter()
        .any(|segment| matches!(*segment, "." | "..") || segment.contains(['/', '\\']))
    {
        return Err(StatusCode::NOT_FOUND);
    }
    fetch_field(storyblok_url(path, &[]), "story").await
}

async fn stories(prefix: &str) -> Result<Vec<Value>, StatusCode> {
    let per_page = STORIES_PER_PAGE.to_string();
    let mut stories = Vec::new();
    for page in 1_u32.. {
        let page = page.to_string();
        let url = storyblok_url(
            &[],
            &[
                ("starts_with", prefix),
                ("per_page", &per_page),
                ("page", &page),
            ],
        );
        let Value::Array(batch) = fetch_field(url, "stories").await? else {
            return Err(StatusCode::BAD_GATEWAY);
        };
        let is_last_page = batch.len() < STORIES_PER_PAGE;
        stories.extend(batch);
        if is_last_page {
            break;
        }
    }
    Ok(stories)
}

pub fn text_contents(value: &Value) -> Vec<&str> {
    let mut texts = Vec::new();
    collect_text_contents(value, &mut texts);
    texts
}

fn collect_text_contents<'a>(value: &'a Value, texts: &mut Vec<&'a str>) {
    match value {
        Value::Object(o) if o.get("component").is_some_and(|c| c == "TextContent") => {
            texts.extend(o.get("text").and_then(Value::as_str));
        }
        Value::Object(o) => o.values().for_each(|v| collect_text_contents(v, texts)),
        Value::Array(a) => a.iter().for_each(|v| collect_text_contents(v, texts)),
        _ => {}
    }
}

fn text(value: &Value) -> String {
    text_contents(value)
        .into_iter()
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn jstr<'a>(v: &'a Value, ptr: &str) -> &'a str {
    v.pointer(ptr).and_then(Value::as_str).unwrap_or("")
}

pub fn md(body: String) -> Response {
    (
        [
            (CONTENT_TYPE, "text/markdown; charset=utf-8"),
            (VARY, "Accept"),
        ],
        body,
    )
        .into_response()
}

pub fn html(body: String) -> Response {
    ([(VARY, "Accept")], Html(body)).into_response()
}

pub async fn not_found() -> Result<Response, StatusCode> {
    Ok((StatusCode::NOT_FOUND, html(fallback()?)).into_response())
}

pub fn wants_markdown(headers: &HeaderMap) -> bool {
    headers
        .get(ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.to_ascii_lowercase().contains("text/markdown"))
}

pub async fn blog_articles() -> Result<Vec<Value>, StatusCode> {
    let mut articles = stories("blog").await?;
    articles.retain(|a| jstr(a, "/full_slug") != "blog/");
    Ok(articles)
}

fn article_links(articles: &[Value]) -> String {
    let base = AP_BASE_URL.as_str();
    articles
        .iter()
        .filter_map(|a| {
            let (slug, title) = (jstr(a, "/full_slug"), jstr(a, "/content/title"));
            (!slug.is_empty() && !title.is_empty())
                .then(|| format!("- [{title}]({base}/{slug}.md)\n"))
        })
        .collect()
}

pub async fn llms_content() -> Result<String, StatusCode> {
    let (home, articles) = tokio::try_join!(story(&["home"]), blog_articles())?;
    Ok(format!(
        "# Aprograma\n\n{}\n\n## Blog\n\n{}",
        text(&home),
        article_links(&articles)
    ))
}

pub async fn home_md_content() -> Result<String, StatusCode> {
    Ok(text(&story(&["home"]).await?))
}

pub async fn blog_md_content() -> Result<String, StatusCode> {
    Ok(format!(
        "# Blog\n\n{}",
        article_links(&blog_articles().await?)
    ))
}

pub async fn article_md_content(slug: &str) -> Result<String, StatusCode> {
    let article = story(&["blog", slug]).await?;
    let body = htmd::convert(jstr(&article, "/content/long_text")).map_err(|error| {
        error!(%error, slug, "Failed to convert article to Markdown");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(format!(
        "# {}\n\n## {}\n\n{body}",
        jstr(&article, "/content/title"),
        jstr(&article, "/content/intro")
    ))
}

pub async fn xml_content(path: &str) -> Result<String, StatusCode> {
    fetch(join(&AP_DATA, &[path.trim_start_matches('/')]))
        .await?
        .text()
        .await
        .map_err(upstream_error)
}
