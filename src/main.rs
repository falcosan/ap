#[macro_use]
mod macros;
mod environment;
mod router;
mod pages {
    export!(home);
    export!(about);
    export!(blog);
    export!(fallback);
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router::router()).await.unwrap();
}
