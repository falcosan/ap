use minijinja::Environment;
use std::sync::LazyLock;

pub(crate) static TEMPLATES: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("layout/default.jinja"))
        .unwrap();
    env.add_template("home.html", include_str!("pages/home.jinja"))
        .unwrap();
    env
});
