use axum::{http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    phone_number: String,
    code: String,
}

pub async fn sign_in(Json(payload): Json<UserForm>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

pub async fn sign_out() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}
