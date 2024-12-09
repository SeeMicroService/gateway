use crate::application;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

pub async fn validate_token(
    State(state): State<application::State>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let token = match headers.get(axum::http::header::AUTHORIZATION) {
        Some(token) => token.to_str().unwrap().trim(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "No authorization header specified"})),
            )
                .into_response()
        }
    };
    let token = match token.strip_prefix("Bearer ") {
        Some(token) => token,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid authorization token"})),
            )
                .into_response()
        }
    };
    match state.auth.validate_token(token).await {
        Ok(valid) => {
            if valid {
                next.run(request).await
            } else {
                (StatusCode::UNAUTHORIZED, Json(Value::default())).into_response()
            }
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}
