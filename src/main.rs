use axum::{ServiceExt, extract::Request};
use std::{
    env,
    io::{self, IsTerminal},
    net::Ipv4Addr,
};
use tokio::{net::TcpListener, signal};
use tower_http::{compression::CompressionLayer, normalize_path::NormalizePathLayer};
use tower_layer::Layer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod environment;
mod http;
mod router;
mod pages {
    pub mod blog;
    pub mod fallback;
    pub mod home;
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_ansi(io::stdout().is_terminal())
        .init();

    http::load_config();

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port))
        .await
        .expect("Failed to bind to address");

    info!("Listening on {}", listener.local_addr().unwrap());

    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash()
            .layer(router::router().layer(CompressionLayer::new())),
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
