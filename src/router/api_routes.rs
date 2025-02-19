use crate::pages::home::increment_counter;
use axum::{routing::post, Router};

fn post_increment_counter(router: Router) -> Router {
    router.route("/increment-counter", post(increment_counter))
}

pub(crate) fn api_routes(mut router: Router) -> Router {
    router = post_increment_counter(router);
    router
}
