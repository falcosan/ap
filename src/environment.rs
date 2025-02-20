#![allow(dead_code)]

use dotenv::dotenv;
use minijinja::{
    functions::Function,
    value::{FunctionArgs, FunctionResult},
    Environment, Template,
};
use std::sync::{LazyLock, Mutex};

pub struct EnvWrapper {
    env: Environment<'static>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        dotenv().ok();

        let mut env = Environment::new();
        let templates = [
            ("layout.html", include_str!("layout/default.jinja")),
            ("home.html", include_str!("pages/home/index.jinja")),
            ("blog.html", include_str!("pages/blog/index.jinja")),
            ("about.html", include_str!("pages/about/index.jinja")),
            ("article.html", include_str!("pages/article/index.jinja")),
            ("fallback.html", include_str!("pages/fallback/index.jinja")),
        ];

        for (name, content) in templates {
            env.add_template(name, content).unwrap();
        }

        Self { env }
    }

    pub fn get_template(&self, name: &str) -> Result<Template, ()> {
        self.env
            .get_template(name)
            .map_err(|_| panic!("Template not found: {}", name))
    }

    pub fn add_function<F, Rv, Args>(&mut self, name: &'static str, func: F)
    where
        F: Function<Rv, Args> + for<'a> Function<Rv, <Args as FunctionArgs<'a>>::Output>,
        Rv: FunctionResult,
        Args: for<'a> FunctionArgs<'a>,
    {
        self.env.add_function(name, func);
    }
}

pub static ENV: LazyLock<Mutex<EnvWrapper>> = LazyLock::new(|| Mutex::new(EnvWrapper::new()));
