use tower_layer::Layer;
use tokio::net::TcpListener;
use std::{ env, net::SocketAddr };
use axum::{ extract::Request, http::Method, ServiceExt };
use tower_http::{ cors::CorsLayer, normalize_path::NormalizePathLayer };

#[macro_use]
mod macros;
mod environment;
mod http;
mod router;
mod pages {
    export!(home);
    export!(blog);
    export!(fallback);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port = env
        ::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());

    let allowed_origins = ["AP_BASE_URL", "DD_BASE_URL"]
        .iter()
        .filter_map(|&var| env::var(var).ok())
        .chain(std::iter::once(addr.to_string()))
        .filter_map(|origin| origin.parse().ok())
        .collect::<Vec<_>>();

    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(
            router
                ::router()
                .layer(CorsLayer::new().allow_methods([Method::GET]).allow_origin(allowed_origins))
        )
    );

    axum::serve(listener, app).await.expect("Server error");
}
