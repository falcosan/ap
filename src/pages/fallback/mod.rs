use crate::environment::render;
use axum::http::StatusCode;

pub fn fallback() -> Result<String, StatusCode> {
    render("fallback.html", "", ())
}
