use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use serde::Deserialize;
use uuid::Uuid;

use crate::service::{
    block::{create_post_by_parent_id, find_block_by_id, update_block_by_modal},
    user::get_user_info_by_id,
};

#[derive(Deserialize)]
pub struct IdPath {
    block_id: Uuid,
}

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
pub struct UpdatePostForm {
    id: Uuid,
    parant_id: Option<Uuid>,
    status: Option<i32>,
    title: Option<String>,
}

pub async fn update_block(
    session: ReadableSession,
    Json(payload): Json<UpdatePostForm>,
) -> (StatusCode, Json<Option<serde_json::Value>>) {
    if let Some(user_id) = session.get::<Uuid>("id") {
        let user = get_user_info_by_id(user_id).await.unwrap();
        match user {
            None => (StatusCode::UNAUTHORIZED, Json(None)),
            Some(user) => {
                let block_result = find_block_by_id(payload.id).await.unwrap();
                match block_result {
                    None => (StatusCode::BAD_REQUEST, Json(None)),
                    Some(block) => {
                        if block.user_id == user.id {
                            let block = update_block_by_modal(
                                block,
                                payload.parant_id,
                                payload.title,
                                Some(1),
                            )
                            .await
                            .unwrap();
                            return (StatusCode::OK, Json(Some(serde_json::json!({ "block": block }))));
                        } else {
                            return (StatusCode::UNAUTHORIZED, Json(None));
                        }
                    }
                }
            }
        }
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn get_block_info(Path(IdPath { block_id }): Path<IdPath>) -> impl IntoResponse {
    let block = find_block_by_id(block_id).await.unwrap();
    match block {
        None => (StatusCode::NOT_FOUND, Json(None)),
        Some(_) => (StatusCode::OK, Json(block)),
    }
}
