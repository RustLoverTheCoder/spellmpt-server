use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use serde_json::json;
use uuid::Uuid;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreatePromptRequest {
    title: String,
    content: String,
}

async fn create_prompt(
    session: ReadableSession,
    req: Json<CreatePromptRequest>,
) -> impl IntoResponse {
    // Check if the session exists and is valid
    if let Some(_) = session.get::<Uuid>("user_id") {
        // Session exists and is valid, proceed with creating the prompt
        let prompt_id = "123";
        (StatusCode::CREATED, Json(json!({ "prompt_id": prompt_id })))
    } else {
        // Session does not exist or is invalid, return unauthorized status code
        (StatusCode::UNAUTHORIZED, Json(json!({})))
    }
}
