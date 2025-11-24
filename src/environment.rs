use serde::Serialize;
use std::{ env, sync::LazyLock };
use minijinja::{ context, Environment, Value };
use chrono::{ Datelike, NaiveDate, NaiveDateTime, Utc };

#[derive(Serialize)]
pub struct PageContext<T> {
    pub data: T,
    pub current_path: String,
}

pub struct EnvWrapper {
    env: Environment<'static>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        let mut env = Environment::new();

        for (name, content) in [
            ("layout.html", include_str!("layout/index.jinja")),
            ("home.html", include_str!("pages/home/index.jinja")),
            ("blog.html", include_str!("pages/blog/index.jinja")),
            ("fallback.html", include_str!("pages/fallback/index.jinja")),
            ("article.html", include_str!("pages/blog/article/index.jinja")),
        ] {
            env.add_template(name, content).expect("duplicate template");
        }

        for (key, value) in [
            ("current_year", Utc::now().year().to_string()),
            ("AP_BASE_URL", env::var("AP_BASE_URL").unwrap_or_default()),
            ("google_verification", env::var("GOOGLE_VERIFICATION").unwrap_or_default()),
        ] {
            env.add_global(key, value);
        }

        env.add_filter("date_format", |v: &str| {
            NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M")
                .map(|dt| dt.date())
                .or_else(|_| NaiveDate::parse_from_str(v, "%Y-%m-%d"))
                .map_or_else(
                    |_| v.to_string(),
                    |d| d.format("%d %b %Y").to_string()
                )
        });

        env.add_filter("startswith", |s: &str, prefix: &str| s.starts_with(prefix));

        Self { env }
    }

    pub fn render_template<T: Serialize>(&self, name: &str, ctx: T) -> String {
        let page = Value::from_serialize(&ctx);
        let current_path = page.get_attr("current_path").unwrap_or_default().to_string();

        self.env
            .get_template(name)
            .and_then(|t| t.render(context!(page => page, current_path => current_path)))
            .or_else(|_| self.env.get_template("fallback.html").and_then(|t| t.render(())))
            .unwrap()
    }
}

pub static ENV: LazyLock<EnvWrapper> = LazyLock::new(EnvWrapper::new);
