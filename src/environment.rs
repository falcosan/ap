use minijinja::value::{FunctionArgs, FunctionResult};
use minijinja::{functions::Function, Environment, Error, Template};
use std::sync::{LazyLock, Mutex};

pub(crate) struct EnvWrapper {
    env: Environment<'static>,
}

impl EnvWrapper {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.add_template("layout.html", include_str!("layout/default.jinja"))
            .expect("Failed to add layout.html template");
        env.add_template("home.html", include_str!("pages/home/index.jinja"))
            .expect("Failed to add home.html template");
        Self { env }
    }

    pub fn get_template(&self, name: &str) -> Result<Template, Error> {
        self.env.get_template(name)
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

pub(crate) static ENV: LazyLock<Mutex<EnvWrapper>> =
    LazyLock::new(|| Mutex::new(EnvWrapper::new()));
