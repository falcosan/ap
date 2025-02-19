mod api_routes;
mod page_routes;

use crate::pages::fallback;
use api_routes::api_routes;
use axum::{http::StatusCode, response::Html, Router};
use page_routes::page_routes;

async fn fallback_handler() -> (StatusCode, Html<String>) {
    (StatusCode::NOT_FOUND, Html(fallback()))
}

pub fn router() -> Router {
    let mut router = Router::new();
    router = page_routes(router);
    router = api_routes(router);
    router.fallback(fallback_handler)
}
