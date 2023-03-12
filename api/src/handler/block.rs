use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn create_block() -> impl IntoResponse {
    return (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "status": "Not Found" })),
    );
}

pub async fn update_block() -> impl IntoResponse {
    return (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "status": "Not Found" })),
    );
}


pub async fn get_block_info() -> impl IntoResponse {
    return (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "status": "Not Found" })),
    );
}
