use crate::application;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use reqwest::header::HeaderMap;
use serde_json::json;

pub async fn refresh(
    State(state): State<application::State>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let token = headers["Authorization"].to_str().unwrap();
    match state.auth.refresh(token).await {
        Ok(token) => (StatusCode::OK, Json(json!({"error": token}))),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}
