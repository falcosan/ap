use axum::http::StatusCode;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use minijinja::{Environment, context, value::Serde};
use serde::Serialize;
use std::{env, sync::LazyLock};
use tracing::error;

static ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();

    for (name, content) in [
        ("layout.html", include_str!("layout/index.jinja")),
        ("home.html", include_str!("pages/home/index.jinja")),
        ("blog.html", include_str!("pages/blog/index.jinja")),
        ("fallback.html", include_str!("pages/fallback/index.jinja")),
        (
            "article.html",
            include_str!("pages/blog/article/index.jinja"),
        ),
    ] {
        env.add_template(name, content)
            .expect("Failed to add template");
    }

    for (key, value) in [
        ("AP_BASE_URL", env::var("AP_BASE_URL").unwrap_or_default()),
        (
            "google_verification",
            env::var("GOOGLE_VERIFICATION").unwrap_or_default(),
        ),
    ] {
        env.add_global(key, value);
    }

    env.add_function("current_year", || Utc::now().year());

    env.add_filter("date_format", |v: &str| {
        NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M")
            .map(|dt| dt.date())
            .or_else(|_| NaiveDate::parse_from_str(v, "%Y-%m-%d"))
            .map_or_else(|_| v.to_string(), |d| d.format("%d %b %Y").to_string())
    });

    env
});

pub fn render<T: Serialize>(name: &str, current_path: &str, data: T) -> Result<String, StatusCode> {
    ENV.get_template(name)
        .and_then(|template| template.render(context! { current_path, data => Serde(data) }))
        .map_err(|error| {
            error!(%error, template = name, "Failed to render template");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
