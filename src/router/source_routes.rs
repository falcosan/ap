use axum::Router;
use tower_http::services::ServeFile;

static STATIC_DIST: &str = "src/static";

pub fn source_routes(router: Router) -> Router {
    router
        .route_service(
            "/index.css",
            ServeFile::new(format!("{}/index.css", STATIC_DIST)),
        )
        .route_service(
            "/favicon.ico",
            ServeFile::new(format!("{}/favicon.ico", STATIC_DIST)),
        )
}
