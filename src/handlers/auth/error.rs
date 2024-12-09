use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::io;
use std::io::ErrorKind;

pub fn handle_error(error: io::Error) -> (StatusCode, Json<Value>) {
    let json_error = Json(json!({"error" : error.to_string()}));
    match error.kind() {
        ErrorKind::NotFound => (StatusCode::NOT_FOUND, json_error),
        ErrorKind::InvalidData => (StatusCode::UNAUTHORIZED, json_error),
        ErrorKind::AlreadyExists => (StatusCode::CONFLICT, json_error),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, json_error),
    }
}
