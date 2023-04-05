use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use uuid::Uuid;

use crate::service::user::{get_user_info_by_id, update_user_info_by_id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserInfoRequest {
    pub nickname: Option<String>,
    pub phone_number: Option<String>,
}

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

pub async fn update_user_info(
    session: ReadableSession,
    req: Json<UpdateUserInfoRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(user_id) = session.get::<Uuid>("id") {
        let user_info = get_user_info_by_id(user_id).await.unwrap();
        match user_info {
            None => return (StatusCode::NOT_FOUND, Json(serde_json::json!({}))),
            Some(_) => {
                let user =
                    update_user_info_by_id(user_id, req.nickname.clone(), req.phone_number.clone())
                        .await
                        .unwrap();
                (StatusCode::OK, Json(serde_json::json!({ "user": user })))
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({})))
    }
}
