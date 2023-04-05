use crate::service::prompt::create_prompt_func;
use axum::{http::StatusCode, response::IntoResponse, Json};
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
    if let Some(_) = session.get::<Uuid>("user_id") {
        // Session exists and is valid, proceed with creating the prompt
        let prompt = create_prompt_func(req.title, req.content, req.prompt_type)
            .await
            .unwrap();
        (StatusCode::CREATED, Json(json!({ "prompt": prompt })))
    } else {
        // Session does not exist or is invalid, return unauthorized status code
        (StatusCode::UNAUTHORIZED, Json(json!({})))
    }
}
