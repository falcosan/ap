use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct PageContext {
    content: String,
}

pub fn get_data_home() -> String {
    let st_token = env::var("ST_TOKEN").expect("Missing ST_TOKEN");

    let response = minreq::get(format!(
        "https://api.storyblok.com/v2/cdn/stories/home?token={}",
        st_token
    ))
    .send()
    .expect("Request failed");

    response
        .as_str()
        .expect("Failed to convert response to string")
        .to_string()
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        content: get_data_home(),
    };
    template.render(context!(page => context)).unwrap()
}
