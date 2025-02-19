use crate::pages::{about, blog, home};
use axum::{response::Html, routing::get, Router};

fn home_handler(router: Router) -> Router {
    router.route("/", get(Html(home())))
}

fn about_handler(router: Router) -> Router {
    router.route("/about", get(Html(about())))
}

fn blog_handler(router: Router) -> Router {
    router.route("/blog", get(Html(blog())))
}

pub(crate) fn page_routes(mut router: Router) -> Router {
    router = home_handler(router);
    router = about_handler(router);
    router = blog_handler(router);
    router
}
