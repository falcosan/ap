mod page_routes;
mod source_routes;

use crate::pages::fallback;
use axum::{ extract::Path, response::{ Html, Redirect }, routing::get, Router };

async fn lang_redirect(path: Option<Path<String>>) -> Redirect {
    Redirect::permanent(&path.map(|Path(p)| format!("/{p}")).unwrap_or("/".into()))
}

pub fn router() -> Router {
    let mut r = Router::new()
        .merge(source_routes::source_routes())
        .merge(page_routes::page_routes());
    for lang in ["it", "es"] {
        r = r
            .route(
                &format!("/{lang}"),
                get(|| lang_redirect(None))
            )
            .route(&format!("/{lang}/{{*path}}"), get(lang_redirect));
    }
    r.fallback(Html(fallback()))
}
