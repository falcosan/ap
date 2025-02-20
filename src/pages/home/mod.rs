use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use serde_json::{from_str, Value};
use std::env;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}
pub fn get_data(name: &str) -> Value {
    let st_token = env::var("ST_TOKEN").unwrap();

    let response = minreq::get(format!(
        "https://api.storyblok.com/v2/cdn/stories/{}?token={}",
        name, st_token
    ))
    .send()
    .expect("Request failed");

    let json: Value = from_str(
        response
            .as_str()
            .expect("Failed to convert response to string"),
    )
    .expect("Failed to convert response to string");

    json["story"].clone()
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        data: get_data("home"),
    };
    template.render(context!(page => context)).unwrap()
}
