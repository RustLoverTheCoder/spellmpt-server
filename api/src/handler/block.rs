use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use serde::Deserialize;
use uuid::Uuid;

use crate::service::{block::create_post_by_parent_id, user::get_user_info_by_id};

#[derive(Deserialize)]
pub struct CreatePostForm {
    parant_id: Option<Uuid>,
}

pub async fn create_block(
    session: ReadableSession,
    Json(payload): Json<CreatePostForm>,
) -> impl IntoResponse {
    if let Some(user_id) = session.get::<Uuid>("id") {
        let user = get_user_info_by_id(user_id).await.unwrap();
        match user {
            None => (StatusCode::UNAUTHORIZED, Json(None)),
            Some(user) => {
                let new_post = create_post_by_parent_id(payload.parant_id, user.id)
                    .await
                    .unwrap();
                (StatusCode::CREATED, Json(Some(new_post)))
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, Json(None))
    }
}

#[derive(Deserialize)]
pub struct updatePostForm {
    id: Uuid,
    parant_id: Option<Uuid>,
    status: Option<i32>,
    title: Option<String>,
}

pub async fn update_block(
    session: ReadableSession,
    Json(payload): Json<updatePostForm>,
) -> impl IntoResponse {
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
