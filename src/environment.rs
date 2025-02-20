#![allow(dead_code)]

use minijinja::{
    functions::Function,
    value::{FunctionArgs, FunctionResult},
    Environment, Template,
};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FMTResult};
use std::sync::{LazyLock, Mutex};

#[derive(Debug)]
pub enum TemplateError {
    NotFound(String),
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        match self {
            TemplateError::NotFound(name) => {
                write!(f, "Template '{}' not found", name)
            }
        }
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TemplateError::NotFound(_) => None,
        }
    }
}

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
            ("about.html", include_str!("pages/about/index.jinja")),
            ("fallback.html", include_str!("pages/fallback/index.jinja")),
        ];

        for (name, content) in templates {
            env.add_template(name, content)
                .unwrap_or_else(|source| panic!("Failed to add template {}: {}", name, source));
        }

        Self { env }
    }

    pub fn get_template(&self, name: &str) -> Result<Template, TemplateError> {
        self.env
            .get_template(name)
            .map_err(|_| TemplateError::NotFound(name.to_string()))
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
