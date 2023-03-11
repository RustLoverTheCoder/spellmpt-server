use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use uuid::Uuid;

use crate::service::user::get_user_info_by_id;

pub async fn get_user_info(session: ReadableSession) -> impl IntoResponse {
    if let Some(user_id) = session.get::<Uuid>("id") {
        let user = get_user_info_by_id(user_id).await.unwrap();
        match user {
            None => (StatusCode::NOT_FOUND, Json(user)),
            Some(_) => (StatusCode::OK, Json(user)),
        }
    } else {
        (StatusCode::UNAUTHORIZED, Json(None))
    }
}

pub async fn update_user_info() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}
