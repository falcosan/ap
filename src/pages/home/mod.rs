use crate::environment::ENV;
use axum::{http::StatusCode, Json};
use serde_json::{from_str, Value};
use std::env;

pub async fn get_data_home() -> Result<Json<Value>, (StatusCode, String)> {
    let st_token = env::var("ST_TOKEN").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Missing ST_TOKEN: {}", e),
        )
    })?;

    let response = minreq::get(format!(
        "https://api.storyblok.com/v2/cdn/stories/home?token={}",
        st_token
    ))
    .send()
    .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Request failed: {}", e)))?;

    let response_str = response.as_str().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Invalid response: {}", e),
        )
    })?;

    let json_response: Value = from_str(response_str).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("JSON parse error: {}", e),
        )
    })?;

    Ok(Json(json_response))
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    template.render(()).unwrap()
}
