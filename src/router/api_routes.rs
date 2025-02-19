use crate::pages::home::{decrease_counter, increase_counter};
use axum::{routing::post, Router};

pub(crate) fn api_routes(router: Router) -> Router {
    router
        .route("/increase-counter", post(increase_counter))
        .route("/decrease-counter", post(decrease_counter))
}
