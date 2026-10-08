use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::Datelike;
use reqwest::{
    Client,
    header::{CONTENT_ENCODING, CONTENT_TYPE, LOCATION, VARY},
    redirect::Policy,
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    net::TcpListener,
    process::{Child, Command},
    time::timeout,
};

const TOKEN: &str = "test-token-3f9a7c";
const ARTICLES: usize = 150;
const RSS: &str = "<rss><channel><title>Feed</title></channel></rss>";
const SITEMAP: &str = "<urlset><url><loc>https://example.com/</loc></url></urlset>";

type Requests = Arc<Mutex<Vec<String>>>;

fn article_story(n: usize) -> Value {
    json!({
        "full_slug": format!("blog/article-{n}"),
        "content": {
            "title": format!("Article {n}"),
            "intro": format!("Intro {n}"),
            "date": "2023-11-14",
            "long_text": format!("<p>Body <strong>{n}</strong></p>"),
            "file": { "filename": "https://img.example.com/a.png", "alt": "Alt" }
        }
    })
}

fn home_story() -> Value {
    json!({
        "full_slug": "home",
        "content": {
            "body": [
                { "component": "TextContent", "text": "# Hello\n\nWorld" },
                { "component": "Grid", "items": [{ "component": "TextContent", "text": "Nested *text*" }] }
            ]
        }
    })
}

fn authorized(query: &HashMap<String, String>) -> bool {
    query.get("token").map(String::as_str) == Some(TOKEN)
}

async fn mock_stories(
    State(requests): State<Requests>,
    uri: Uri,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    requests.lock().unwrap().push(uri.to_string());
    if !authorized(&query) || query.get("starts_with").map(String::as_str) != Some("blog") {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let per_page = query
        .get("per_page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(25_usize)
        .min(100);
    let page = query
        .get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_usize);
    let stories: Vec<Value> = std::iter::once(json!({ "full_slug": "blog/", "content": {} }))
        .chain((1..=ARTICLES).map(article_story))
        .skip((page - 1) * per_page)
        .take(per_page)
        .collect();
    Json(json!({ "stories": stories })).into_response()
}

async fn mock_story(
    State(requests): State<Requests>,
    uri: Uri,
    Path(path): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    requests.lock().unwrap().push(uri.to_string());
    if !authorized(&query) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let mut segments = Vec::new();
    for segment in path.split(['/', '\\']) {
        match segment {
            ".." => {
                segments.pop();
            }
            "" | "." => {}
            segment => segments.push(segment),
        }
    }
    let slug = segments.join("/");
    let story = if slug == "home" {
        Some(home_story())
    } else {
        slug.strip_prefix("blog/article-")
            .and_then(|n| n.parse().ok())
            .filter(|n| (1..=ARTICLES).contains(n))
            .map(article_story)
    };
    story.map_or_else(
        || StatusCode::NOT_FOUND.into_response(),
        |story| Json(json!({ "story": story })).into_response(),
    )
}

async fn mock_data(Path(file): Path<String>) -> Response {
    match file.as_str() {
        "rss.xml" => RSS.into_response(),
        "sitemap.xml" => SITEMAP.into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn spawn_upstream() -> (String, Requests) {
    let requests = Requests::default();
    let router = Router::new()
        .route("/v2/cdn/stories", get(mock_stories))
        .route("/v2/cdn/stories/{*path}", get(mock_story))
        .route("/data/{file}", get(mock_data))
        .with_state(requests.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    (format!("http://{address}"), requests)
}

async fn closed_port_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    format!("http://{}", listener.local_addr().unwrap())
}

fn command(dir: PathBuf, env: &[(&str, &str)]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ap"));
    command
        .current_dir(dir)
        .env_clear()
        .envs(env.iter().copied())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    command
}

fn collect(stream: impl AsyncRead + Unpin + Send + 'static, logs: Arc<Mutex<String>>) {
    tokio::spawn(async move {
        let mut lines = BufReader::new(stream).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let mut logs = logs.lock().unwrap();
            logs.push_str(&line);
            logs.push('\n');
        }
    });
}

struct App {
    url: String,
    child: Child,
    logs: Arc<Mutex<String>>,
}

impl App {
    async fn spawn(upstream: &str) -> Self {
        Self::spawn_with(upstream, upstream).await
    }

    async fn spawn_with(storyblok: &str, data: &str) -> Self {
        let base_url = format!("{storyblok}/v2/cdn/stories");
        let data_url = format!("{data}/data/");
        let mut child = command(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            &[
                ("PORT", "0"),
                ("RUST_LOG", "info"),
                ("ST_TOKEN", TOKEN),
                ("ST_BASE_URL", &base_url),
                ("AP_DATA", &data_url),
                ("AP_BASE_URL", "https://example.com/"),
                ("GOOGLE_VERIFICATION", ""),
            ],
        )
        .spawn()
        .unwrap();

        let logs = Arc::new(Mutex::new(String::new()));
        collect(child.stderr.take().unwrap(), logs.clone());
        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
        let address: SocketAddr = timeout(Duration::from_secs(10), async {
            loop {
                let line = lines.next_line().await.unwrap().expect("server exited");
                if let Some((_, address)) = line.split_once("Listening on ") {
                    break address.parse().unwrap();
                }
            }
        })
        .await
        .expect("server did not start");
        collect(lines.into_inner(), logs.clone());

        Self {
            url: format!("http://127.0.0.1:{}", address.port()),
            child,
            logs,
        }
    }

    async fn get(&self, path: &str) -> reqwest::Response {
        self.get_with(path, &[]).await
    }

    async fn get_with(&self, path: &str, headers: &[(&str, &str)]) -> reqwest::Response {
        let client = Client::builder()
            .redirect(Policy::none())
            .no_brotli()
            .no_gzip()
            .build()
            .unwrap();
        headers
            .iter()
            .fold(
                client.get(format!("{}{path}", self.url)),
                |request, (name, value)| request.header(*name, *value),
            )
            .send()
            .await
            .unwrap()
    }

    async fn text(&self, path: &str) -> String {
        let response = self.get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        response.text().await.unwrap()
    }

    async fn wait_for_log(&self, needle: &str) -> String {
        timeout(Duration::from_secs(5), async {
            loop {
                let logs = self.logs.lock().unwrap().clone();
                if logs.contains(needle) {
                    break logs;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("log line not found")
    }
}

async fn run_to_exit(env: &[(&str, &str)]) -> (ExitStatus, String) {
    let output = timeout(
        Duration::from_secs(10),
        command(std::env::temp_dir(), env).output(),
    )
    .await
    .expect("process did not exit")
    .unwrap();
    (
        output.status,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[tokio::test]
async fn home_renders_markdown_sections() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    let response = app.get("/").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "text/html; charset=utf-8");
    assert_eq!(response.headers()[VARY], "Accept");
    let body = response.text().await.unwrap();

    assert!(body.contains("<section><h1>Hello</h1>\n<p>World</p>\n</section>"));
    assert!(body.contains("<section><p>Nested <em>text</em></p>\n</section>"));
    assert!(body.contains(r#"<a href="/" aria-current="page">Home</a>"#));
    assert!(body.contains(&format!("&copy; {}, Aprograma", chrono::Utc::now().year())));
    assert!(body.contains(r#"rel="alternate" type="application/rss+xml""#));
    assert!(!body.contains("google-site-verification"));
    assert!(body.trim_start().starts_with("<!DOCTYPE html>"));
    assert!(body.find(r#"<meta charset="UTF-8" />"#).unwrap() < body.find("<title>").unwrap());
}

#[tokio::test]
async fn home_serves_markdown_by_extension_and_accept_header() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;
    let expected = "# Hello\n\nWorld\n\nNested *text*";

    let by_extension = app.get("/.md").await;
    assert_eq!(
        by_extension.headers()[CONTENT_TYPE],
        "text/markdown; charset=utf-8"
    );
    assert_eq!(by_extension.text().await.unwrap(), expected);

    let by_header = app.get_with("/", &[("accept", "text/markdown")]).await;
    assert_eq!(by_header.status(), StatusCode::OK);
    assert_eq!(
        by_header.headers()[CONTENT_TYPE],
        "text/markdown; charset=utf-8"
    );
    assert_eq!(by_header.text().await.unwrap(), expected);
}

#[tokio::test]
async fn blog_lists_every_article_across_pages() {
    let (upstream, requests) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    let html = app.text("/blog").await;
    assert_eq!(
        html.matches(r#"<a href="/blog&#x2f;article-"#).count(),
        ARTICLES
    );
    assert!(html.contains(r#"<a href="/blog&#x2f;article-150">Article 150</a>"#));
    assert!(html.contains(r#"<a href="/blog" aria-current="page">Blog</a>"#));
    assert!(!html.contains(r#"href="/blog&#x2f;""#));

    let markdown = app.text("/blog.md").await;
    let lines: Vec<&str> = markdown.lines().filter(|l| l.starts_with("- ")).collect();
    assert_eq!(lines.len(), ARTICLES);
    assert_eq!(
        lines[0],
        "- [Article 1](https://example.com/blog/article-1.md)"
    );
    assert!(markdown.starts_with("# Blog\n\n"));

    let pages: Vec<String> = requests
        .lock()
        .unwrap()
        .iter()
        .filter(|r| r.starts_with("/v2/cdn/stories?"))
        .cloned()
        .collect();
    assert!(pages.iter().all(|r| r.contains("per_page=100")));
    assert!(pages.iter().any(|r| r.contains("page=2")));

    assert_eq!(app.text("/blog/").await, html);
    assert_eq!(
        app.get_with("/blog", &[("accept", "text/markdown")])
            .await
            .text()
            .await
            .unwrap(),
        markdown
    );
}

#[tokio::test]
async fn article_renders_html() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    let html = app.text("/blog/article-7").await;
    assert!(html.contains("<title>Article 7</title>"));
    assert!(html.contains("<h1>Article 7</h1>"));
    assert!(html.contains(r#"<time datetime="2023-11-14"><small>14 Nov 2023</small></time>"#));
    assert!(html.contains(r#"fetchpriority="high""#));
    assert!(!html.contains(r#"loading="lazy""#));
    assert!(html.contains("<p>Body <strong>7</strong></p>"));
    assert!(html.contains(r#"<a href="/blog" aria-current="true">Blog</a>"#));
    assert!(html.contains(
        r#"<link rel="canonical" href="https:&#x2f;&#x2f;example.com&#x2f;blog&#x2f;article-7">"#
    ));
}

#[tokio::test]
async fn article_serves_markdown() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;
    let expected = "# Article 7\n\n## Intro 7\n\nBody **7**";

    assert_eq!(app.text("/blog/article-7.md").await, expected);
    let by_header = app
        .get_with("/blog/article-7", &[("accept", "text/markdown")])
        .await;
    assert_eq!(by_header.text().await.unwrap(), expected);
}

#[tokio::test]
async fn missing_pages_return_not_found() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    for path in ["/nope", "/blog/missing", "/blog/article-151"] {
        let response = app.get(path).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
        assert_eq!(response.headers()[CONTENT_TYPE], "text/html; charset=utf-8");
        assert!(
            response
                .text()
                .await
                .unwrap()
                .contains("404: Page not found")
        );
    }
    assert_eq!(
        app.get("/blog/missing.md").await.status(),
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn path_traversal_never_reaches_upstream() {
    let (upstream, requests) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    for path in [
        "/blog/..%2Fhome",
        "/blog/..%2Fhome.md",
        "/blog/%2E%2E%2Fhome.md",
        "/blog/..%5Chome.md",
        "/blog/..%2F..%2Fstories%2Fhome.md",
    ] {
        assert_eq!(
            app.get(path).await.status(),
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }
    assert!(requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn slug_cannot_inject_upstream_query() {
    let (upstream, requests) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    assert_eq!(
        app.get("/blog/x%3Fversion%3Ddraft%23.md").await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        [format!(
            "/v2/cdn/stories/blog/x%3Fversion=draft%23?token={TOKEN}"
        )]
    );
}

#[tokio::test]
async fn language_prefixes_redirect_within_the_site() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    for (path, location) in [
        ("/it", "/"),
        ("/es/blog", "/blog"),
        ("/it/blog/article-1", "/blog/article-1"),
        ("/it//evil.com", "/evil.com"),
        ("/es///evil.com", "/evil.com"),
        ("/it/%2F%2Fevil.com", "/%2F%2Fevil.com"),
        ("/es/%5Cevil.com", "/%5Cevil.com"),
        ("/it/%09/evil.com", "/%09/evil.com"),
    ] {
        let response = app.get(path).await;
        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT, "{path}");
        assert_eq!(response.headers()[LOCATION], location, "{path}");
    }
}

#[tokio::test]
async fn feeds_and_llms_txt_are_served() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    for (path, body) in [("/rss.xml", RSS), ("/sitemap.xml", SITEMAP)] {
        let response = app.get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_eq!(response.headers()[CONTENT_TYPE], "application/xml");
        assert_eq!(response.text().await.unwrap(), body);
    }

    let llms = app.text("/llms.txt").await;
    assert!(llms.starts_with(
        "# Aprograma\n\n# Hello\n\nWorld\n\nNested *text*\n\n## Blog\n\n- [Article 1](https://example.com/blog/article-1.md)\n"
    ));
    assert_eq!(
        llms.lines()
            .filter(|l| l.starts_with("- [Article "))
            .count(),
        ARTICLES
    );
}

#[tokio::test]
async fn static_files_are_served_compressed() {
    let (upstream, _) = spawn_upstream().await;
    let app = App::spawn(&upstream).await;

    for (path, content_type) in [
        ("/index.css", "text/css"),
        ("/robots.txt", "text/plain"),
        ("/favicon.ico", "image/x-icon"),
    ] {
        let response = app.get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_eq!(response.headers()[CONTENT_TYPE], content_type, "{path}");
    }

    let compressed = app.get_with("/", &[("accept-encoding", "br")]).await;
    assert_eq!(compressed.headers()[CONTENT_ENCODING], "br");
}

#[tokio::test]
async fn upstream_failures_return_bad_gateway_without_leaking_the_token() {
    let closed = closed_port_url().await;
    let app = App::spawn_with(&closed, &closed).await;

    for path in [
        "/",
        "/.md",
        "/blog",
        "/blog.md",
        "/blog/article-1",
        "/blog/article-1.md",
        "/llms.txt",
        "/rss.xml",
        "/sitemap.xml",
    ] {
        assert_eq!(
            app.get(path).await.status(),
            StatusCode::BAD_GATEWAY,
            "{path}"
        );
    }
    assert_eq!(app.get("/nope").await.status(), StatusCode::NOT_FOUND);

    let logs = app.wait_for_log(r#"path="/data/sitemap.xml""#).await;
    assert!(logs.contains(r#"Upstream request failed path="/v2/cdn/stories/home""#));
    assert!(!logs.contains(TOKEN));
    assert!(!logs.contains('\u{1b}'));
}

#[cfg(unix)]
#[tokio::test]
async fn sigterm_shuts_down_gracefully() {
    let (upstream, _) = spawn_upstream().await;
    let mut app = App::spawn(&upstream).await;
    let pid = app.child.id().unwrap().to_string();

    let kill = Command::new("kill")
        .args(["-TERM", &pid])
        .status()
        .await
        .unwrap();
    assert!(kill.success());
    let status = timeout(Duration::from_secs(5), app.child.wait())
        .await
        .expect("server ignored SIGTERM")
        .unwrap();
    assert!(status.success());
}

#[tokio::test]
async fn missing_or_invalid_config_fails_at_startup() {
    let (status, stderr) = run_to_exit(&[("PORT", "0")]).await;
    assert!(!status.success());
    assert!(stderr.contains("ST_TOKEN not set"));

    let (status, stderr) = run_to_exit(&[
        ("PORT", "0"),
        ("ST_TOKEN", TOKEN),
        ("ST_BASE_URL", "not-a-url"),
        ("AP_DATA", "https://example.com/"),
    ])
    .await;
    assert!(!status.success());
    assert!(stderr.contains("ST_BASE_URL is not a valid URL"));
}
