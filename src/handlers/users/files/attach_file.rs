use crate::application;
use crate::domain::dto::File;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use crate::handlers::error::handle_error;
use crate::handlers::put_file;

pub async fn attach_file(
    State(state): State<application::State>,
    Json(file): Json<File>,
) -> (StatusCode, Json<Value>) {
    if let Some(ref id) = file.id {
        if let Err(err) = state.users.attach_file(id, &file.name).await {
            let _ = state.users.detach_file(id, &file.name).await;
            let error_message = Json(json!({"error": err.to_string()}));
            return handle_error(err, error_message);
        }
        put_file(State(state), Json(file)).await
    } else {
        (StatusCode::BAD_REQUEST, Json(json!({"error": "You must provide a user id"})))
    }
}
