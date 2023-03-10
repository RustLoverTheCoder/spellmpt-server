use axum::{response::Html, routing::get, Router};
use axum_sessions::SessionLayer;
use reels_config::{
    contants::{JWT_SECRET, REDIS_SESSION_STORE},
    init,
};
use std::net::SocketAddr;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    init().await?;
    let jwt_secret = JWT_SECRET.get().unwrap();
    let jwt_secret_bytes = jwt_secret.as_bytes();
    let store = REDIS_SESSION_STORE.get().unwrap().clone();
    let session_layer = SessionLayer::new(store, jwt_secret_bytes).with_secure(false);
    let app = Router::new().route("/", get(handler)).layer(session_layer);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("addr:{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
