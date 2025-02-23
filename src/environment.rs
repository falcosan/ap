use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use minijinja::{Environment, Template};
use std::{env, sync::LazyLock};

pub struct EnvWrapper {
    env: Environment<'static>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        let mut env = Environment::new();
        let templates = [
            ("layout.html", include_str!("layout/default.jinja")),
            ("home.html", include_str!("pages/home/index.jinja")),
            ("blog.html", include_str!("pages/blog/index.jinja")),
            ("fallback.html", include_str!("pages/fallback/index.jinja")),
            (
                "article.html",
                include_str!("pages/blog/article/index.jinja"),
            ),
        ];

        for (name, content) in templates {
            env.add_template(name, content).unwrap();
        }
        env.add_global("current_year", Utc::now().year().to_string());
        env.add_global("origin_dd", env::var("ORIGIN_DD").unwrap_or_default());
        env.add_filter("date_format", |v: &str| {
            NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M")
                .map(|dt| dt.date())
                .or_else(|_| NaiveDate::parse_from_str(v, "%Y-%m-%d"))
                .map_or_else(|_| v.to_string(), |d| d.format("%d %b %Y").to_string())
        });

        Self { env }
    }

    pub fn get_template(&self, name: &str) -> Template {
        self.env
            .get_template(name)
            .unwrap_or_else(|err| panic!("Template not found ({}): {:?}", name, err))
    }
}

pub static ENV: LazyLock<EnvWrapper> = LazyLock::new(|| EnvWrapper::new());
