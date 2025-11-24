use axum::Router;
use tower_http::services::ServeFile;

pub fn source_routes(router: Router) -> Router {
    router
        .route_service("/index.css", ServeFile::new("src/static/index.css"))
        .route_service("/robots.txt", ServeFile::new("src/static/robots.txt"))
        .route_service("/favicon.ico", ServeFile::new("src/static/favicon.ico"))
}
