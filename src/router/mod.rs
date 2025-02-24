mod page_routes;
mod source_routes;

use crate::pages::fallback;
use axum::{response::Html, Router};
use page_routes::page_routes;
use source_routes::source_routes;

pub fn router() -> Router {
    let mut router = Router::new();
    router = source_routes(router);
    router = page_routes(router);
    router.fallback(Html(fallback()))
}
