use axum::Router;
use tower_http::services::ServeFile;

pub(crate) fn source_routes(router: Router) -> Router {
    router.route_service("/index.css", ServeFile::new("src/assets/index.css"))
}
