use crate::application;
use crate::domain::dto::UserId;
use crate::handlers::error::handle_error;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn get_filenames(
    State(state): State<application::State>,
    Query(id): Query<UserId>,
) -> (StatusCode, Json<Value>) {
    match state.users.get_filenames(&id).await {
        Ok(files) => (StatusCode::OK, Json(json!({"filenames": files}))),
        Err(error) => {
            let error_message = Json(json!({ "error": "Invalid id" }));
            handle_error(error, error_message)
        }
    }
}
