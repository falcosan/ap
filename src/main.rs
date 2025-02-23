use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[macro_use]
mod macros;
mod environment;
mod router;
mod pages {
    export!(home);
    export!(blog);
    export!(fallback);
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());

    let allowed_origins =
        std::iter::once(addr.to_string().parse().expect("Invalid default origin"))
            .chain(["ORIGIN_AP", "ORIGIN_DD"].iter().map(|&var| {
                env::var(var)
                    .unwrap_or_else(|_| panic!("Missing environment variable: {}", var))
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid origin provided in {}", var))
            }))
            .collect::<Vec<_>>();
    let app = router::router().layer(CorsLayer::new().allow_origin(allowed_origins));

    axum::serve(listener, app).await.expect("Server error");
}
