use crate::application;
use crate::domain::dto::{File, Path};
use crate::handlers::delete_file;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn detach_file(
    State(state): State<application::State>,
    Json(file): Json<File>,
) -> (StatusCode, Json<Value>) {
    if let Some(ref id) = file.id {
        if let Err(err) = state.users.detach_file(id, &file.name).await {
            let _ = state.users.attach_file(id, &file.name).await;
            let error_message = Json(json!({"error": err.to_string()}));
            return handle_error(err, error_message);
        }
        delete_file(State(state), Json(Path { path: file.name })).await
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You must provide a user id"})),
        )
    }
}
