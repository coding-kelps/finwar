use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0";
    let port = 4444;
    let app = Router::new().route("/", get(|| async { "Welcome to FinWar!" }))
        .route("/ranking", get(ranking))
        .route("/buy", get(buy))
        .route("/sell", get(sell))
        .route("/price", get(price));

    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}")).await.expect("could not bind to address");
    println!("Listening on http://{addr}:{port}");
    axum::serve(listener, app).await.expect("could not start server");
}

async fn ranking() -> &'static str {
    "ranking"
}
async fn buy() -> &'static str {
    "buy"
}
async fn sell() -> &'static str {
    "sell"
}
async fn price() -> &'static str {
    "price"
}