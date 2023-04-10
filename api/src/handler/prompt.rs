use crate::service::prompt::{create_prompt_func, find_prompt_by_id_func};
use axum::{extract::Path, http::StatusCode, Json};
use axum_sessions::extractors::ReadableSession;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePromptRequest {
    title: String,
    content: String,
    prompt_type: i32,
}

pub async fn create_prompt(
    session: ReadableSession,
    Json(req): Json<CreatePromptRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Check if the session exists and is valid
    if let Some(user_id) = session.get::<Uuid>("id") {
        // Session exists and is valid, proceed with creating the prompt
        let prompt = create_prompt_func(req.title, req.content, req.prompt_type, user_id)
            .await
            .unwrap();

        (StatusCode::CREATED, Json(json!({ "prompt": prompt })))
    } else {
        // Session does not exist or is invalid, return unauthorized status code
        (StatusCode::UNAUTHORIZED, Json(json!({})))
    }
}

#[derive(Deserialize)]
pub struct IdPath {
    prompt_id: Uuid,
}

pub async fn find_prompt(
    Path(IdPath { prompt_id }): Path<IdPath>,
) -> (StatusCode, Json<serde_json::Value>) {
    let prompt = find_prompt_by_id_func(prompt_id).await.unwrap();

    match prompt {
        Some(prompt) => (StatusCode::OK, Json(json!({ "prompt": prompt }))),
        None => (StatusCode::NOT_FOUND, Json(json!({}))),
    }
}
