mod handler;
mod service;

use axum::{
    routing::{get, post},
    Router,
};
use axum_sessions::SessionLayer;
use handler::{
    auth::{sign_in, sign_out},
    user::{get_user_info, update_user_info},
};
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
    let auth_router = Router::new()
        .route("/sign/in", post(sign_in))
        .route("/sign/out", post(sign_out));

    let user_router = Router::new()
        .route("/get/info", get(get_user_info))
        .route("/update/info", post(update_user_info));
    let api_routes = Router::new()
        .nest("/auth", auth_router)
        .nest("/user", user_router);

    let app = Router::new().nest("/api", api_routes).layer(session_layer);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("addr:{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
