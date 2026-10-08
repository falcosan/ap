mod page_routes;
mod source_routes;

use crate::http::not_found;
use axum::{Router, http::Uri, response::Redirect, routing::get};

async fn lang_redirect(uri: Uri) -> Redirect {
    let path = uri.path().splitn(3, '/').nth(2).unwrap_or_default();
    Redirect::permanent(&format!("/{}", path.trim_start_matches(['/', '\\'])))
}

pub fn router() -> Router {
    let mut r = Router::new()
        .merge(source_routes::source_routes())
        .merge(page_routes::page_routes());
    for lang in ["it", "es"] {
        r = r
            .route(&format!("/{lang}"), get(lang_redirect))
            .route(&format!("/{lang}/{{*path}}"), get(lang_redirect));
    }
    r.fallback(not_found)
}
