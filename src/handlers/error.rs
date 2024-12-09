use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::io;

pub fn handle_error(error: io::Error, message: Json<Value>) -> (StatusCode, Json<Value>) {
    match error.kind() {
        io::ErrorKind::InvalidData => (StatusCode::BAD_REQUEST, message),
        io::ErrorKind::NotFound => (StatusCode::NOT_FOUND, message),
        io::ErrorKind::AlreadyExists => (StatusCode::CONFLICT, message),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}
