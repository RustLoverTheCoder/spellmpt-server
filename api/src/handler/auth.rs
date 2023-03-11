use axum::{http::StatusCode, Json};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    phone_number: String,
    code: String,
}

pub async fn login(
    mut session: WritableSession,
    Json(payload): Json<UserForm>,
) -> (StatusCode, Json<serde_json::Value>) {
    
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

pub async fn login_out(mut session: WritableSession) -> (StatusCode, Json<serde_json::Value>) {
    session.destroy();
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}
