use axum::Router;
use tower_http::services::ServeFile;

static STATIC_DIST: &str = "src/static";

pub fn source_routes(router: Router) -> Router {
    router
        .route_service(
            "/index.css",
            ServeFile::new(format!("{STATIC_DIST}/index.css")),
        )
        .route_service(
            "/robots.txt",
            ServeFile::new(format!("{STATIC_DIST}/robots.txt")),
        )
        .route_service(
            "/favicon.ico",
            ServeFile::new(format!("{STATIC_DIST}/favicon.ico")),
        )
}
