use minijinja::Environment;
use std::sync::OnceLock;

static ENV: OnceLock<Environment<'static>> = OnceLock::new();

pub(crate) fn templates() -> &'static Environment<'static> {
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.add_template("layout.html", include_str!("layout/default.jinja"))
            .unwrap();
        env.add_template("home.html", include_str!("pages/home.jinja"))
            .unwrap();
        env
    })
}
