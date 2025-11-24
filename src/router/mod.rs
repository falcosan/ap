mod page_routes;
mod source_routes;

use crate::pages::fallback;
use axum::{ extract::Path, response::{ Html, Redirect }, routing::get, Router };

pub fn router() -> Router {
    Router::new()
        .merge(source_routes::source_routes(Router::new()))
        .merge(page_routes::page_routes(Router::new()))
        .route(
            "/it",
            get(|| async { Redirect::permanent("/") })
        )
        .route(
            "/es",
            get(|| async { Redirect::permanent("/") })
        )
        .route(
            "/it/{*path}",
            get(|Path(p): Path<String>| async move { Redirect::permanent(&format!("/{p}")) })
        )
        .route(
            "/es/{*path}",
            get(|Path(p): Path<String>| async move { Redirect::permanent(&format!("/{p}")) })
        )
        .fallback(Html(fallback()))
}
