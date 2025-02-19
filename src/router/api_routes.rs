use axum::{response::Html, routing::post, Router};

async fn increment_counter() -> Html<String> {
    Html("Hello, World!".to_string())
}

pub(crate) fn api_routes(router: Router) -> Router {
    router.route("/increment-counter", post(increment_counter))
}
