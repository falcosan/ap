use crate::environment::ENV;
use axum::Json;
use serde_json::{json, Value};
use std::env;

pub async fn get_data_home() -> Json<Value> {
    let st_token = env::var("ST_TOKEN");
    let response = reqwest::get(format!(
        "https://api.storyblok.com/v2/cdn/stories/home?token={}",
        st_token.unwrap()
    ))
    .await;

    match response {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(parsed) => Json(parsed),
            Err(e) => Json(json!({"error": format!("JSON parsing failed: {}", e)})),
        },
        Err(e) => Json(json!({"error": format!("Request failed: {}", e)})),
    }
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    template.render(()).unwrap()
}
