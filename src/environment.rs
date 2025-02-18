use minijinja::value::{FunctionArgs, FunctionResult};
use minijinja::{functions::Function, Environment, Error as MJError, Template};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FMTResult};
use std::sync::{LazyLock, Mutex};

#[derive(Debug)]
pub enum TemplateError {
    Addition { name: String, source: MJError },
    NotFound(String),
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        match self {
            TemplateError::Addition { name, source } => {
                write!(f, "Error adding template '{}': {}", name, source)
            }
            TemplateError::NotFound(name) => {
                write!(f, "Template '{}' not found", name)
            }
        }
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TemplateError::Addition { source, .. } => Some(source),
            TemplateError::NotFound(_) => None,
        }
    }
}

pub struct EnvWrapper {
    env: Environment<'static>,
    templates: HashMap<&'static str, &'static str>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            templates: HashMap::from([
                ("layout.html", include_str!("layout/default.jinja")),
                ("home.html", include_str!("pages/home/index.jinja")),
                ("blog.html", include_str!("pages/blog/index.jinja")),
                ("about.html", include_str!("pages/about/index.jinja")),
                ("fallback.html", include_str!("pages/fallback/index.jinja")),
            ]),
        }
    }
    pub fn initialize(&mut self) -> Result<(), TemplateError> {
        for (name, content) in &self.templates {
            self.env
                .add_template(name, content)
                .map_err(|source| TemplateError::Addition {
                    name: name.to_string(),
                    source,
                })?;
        }
        Ok(())
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

pub static ENV: LazyLock<Mutex<EnvWrapper>> = LazyLock::new(|| {
    let mut wrapper = EnvWrapper::new();
    wrapper
        .initialize()
        .expect("Failed to initialize templates");
    Mutex::new(wrapper)
});
