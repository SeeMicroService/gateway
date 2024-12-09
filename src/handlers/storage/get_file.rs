use crate::application;
use crate::domain::dto::Path;
use crate::handlers::error::handle_error;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

pub async fn get_file(
    State(state): State<application::State>,
    Query(path): Query<Path>,
) -> (StatusCode, Body) {
    match state.storage.get_file(&path).await {
        Ok(file) => (StatusCode::OK, Body::from(file)),
        Err(error) => {
            let error_msg = Json(json!({"error": "No such file or directory"}));
            let (code, body) = handle_error(error, error_msg);
            (code, Body::from(body.to_string()))
        }
    }
}
