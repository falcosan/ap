use axum::Router;
use tower_http::services::ServeFile;

static ASSETS_DIST: &str = "src/assets";

pub fn source_routes(router: Router) -> Router {
    router
        .route_service(
            "/index.css",
            ServeFile::new(format!("{}/index.css", ASSETS_DIST)),
        )
        .route_service(
            "/favicon.ico",
            ServeFile::new(format!("{}/favicon.ico", ASSETS_DIST)),
        )
}
