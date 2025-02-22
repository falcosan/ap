use std::env;

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
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind listener");
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router::router()).await.unwrap();
}
