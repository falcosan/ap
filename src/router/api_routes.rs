use axum::{routing::get, Router};

pub(crate) fn api_routes(router: Router) -> Router {
    router.route("/get_data_home", get(()))
}
