use axum::{http::StatusCode, Json};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde::Deserialize;

use crate::service::user::find_user_by_phone;

#[derive(Deserialize)]
pub struct UserForm {
    phone_number: String,
    code: String,
}

pub async fn login(
    mut session: WritableSession,
    Json(payload): Json<UserForm>,
) -> (StatusCode, Json<serde_json::Value>) {
    find_user_by_phone(payload.phone_number).await;
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
