use axum::{http::StatusCode, Json};
use axum_sessions::extractors::WritableSession;
use serde::Deserialize;

use crate::service::user::{create_user, find_user_by_phone};

#[derive(Deserialize)]
pub struct UserForm {
    phone_number: String,
    // code: String,
}

pub async fn login(
    mut session: WritableSession,
    Json(payload): Json<UserForm>,
) -> (StatusCode, Json<serde_json::Value>) {
    let phone = payload.phone_number.clone();
    if let Some(user) = find_user_by_phone(payload.phone_number).await.unwrap() {
        session.insert("id", user.id).unwrap();
        (StatusCode::OK, Json(serde_json::json!({ "user": user })))
    } else {
        let new_user = create_user(phone).await.unwrap();
        session.insert("id", new_user.id).unwrap();
        (
            StatusCode::OK,
            Json(serde_json::json!({ "user": new_user })),
        )
    }
}

pub async fn login_out(mut session: WritableSession) -> (StatusCode, Json<serde_json::Value>) {
    session.destroy();
    (
        StatusCode::NO_CONTENT,
        Json(serde_json::json!({ "success": true })),
    )
}
