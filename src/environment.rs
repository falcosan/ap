use std::{ env, sync::LazyLock };
use minijinja::{ context, Value, Environment };
use chrono::{ Utc, Datelike, NaiveDate, NaiveDateTime };

pub struct EnvWrapper {
    env: Environment<'static>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        let mut env = Environment::new();
        let templates = [
            ("layout.html", include_str!("layout/index.jinja")),
            ("home.html", include_str!("pages/home/index.jinja")),
            ("blog.html", include_str!("pages/blog/index.jinja")),
            ("fallback.html", include_str!("pages/fallback/index.jinja")),
            ("article.html", include_str!("pages/blog/article/index.jinja")),
        ];
        let globals = [
            ("current_year", Utc::now().year().to_string()),
            ("AP_BASE_URL", env::var("AP_BASE_URL").unwrap_or_default()),
            ("google_verification", env::var("GOOGLE_VERIFICATION").unwrap_or_default()),
        ];

        for (name, content) in templates {
            env.add_template(name, content).unwrap_or_else(|_|
                panic!("Template already exist: {name}")
            );
        }
        for (key, value) in globals {
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

    pub fn render_template<T: serde::Serialize>(&self, name: &str, context: T) -> String {
        let page = Value::from_serialize(&context);
        let current_path = page.get_attr("current_path").unwrap_or_default().to_string();

        self.env
            .get_template(name)
            .and_then(|template| {
                template.render(context!(page => page, current_path => current_path))
            })
            .or_else(|_| { self.env.get_template("fallback.html").and_then(|t| t.render(())) })
            .unwrap()
    }
}

pub static ENV: LazyLock<EnvWrapper> = LazyLock::new(EnvWrapper::new);
