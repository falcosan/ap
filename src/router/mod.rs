mod page_routes;
mod source_routes;

use crate::pages::fallback;
use axum::{
    extract::Path,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use page_routes::page_routes;
use source_routes::source_routes;

async fn redirect_to_root() -> Redirect {
    Redirect::permanent("/")
}

async fn redirect_lang_path(Path(path): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/{path}"))
}

pub fn router() -> Router {
    Router::new()
        .merge(source_routes(Router::new()))
        .merge(page_routes(Router::new()))
        .route("/it", get(redirect_to_root))
        .route("/es", get(redirect_to_root))
        .route("/it/{*path}", get(redirect_lang_path))
        .route("/es/{*path}", get(redirect_lang_path))
        .fallback(Html(fallback()))
}
