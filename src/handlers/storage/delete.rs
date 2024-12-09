use crate::application;
use crate::domain::dto::Path;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn delete_file(
    State(state): State<application::State>,
    Json(path): Json<Path>,
) -> (StatusCode, Json<Value>) {
    match state.storage.delete(&path).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(error) => {
            let error_msg = Json(json!({"error": "No such file or directory"}));
            handle_error(error, error_msg)
        }
    }
}
