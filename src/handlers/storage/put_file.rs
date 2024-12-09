use crate::application;
use crate::domain::dto::File;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn put_file(
    State(state): State<application::State>,
    Json(file): Json<File>,
) -> (StatusCode, Json<Value>) {
    if let Err(error) = state.storage.put(&file).await {
        let error_msg = Json(json!({"error": error.to_string()}));
        handle_error(error, error_msg)
    } else {
        (StatusCode::CREATED, Json(json!({})))
    }
}
