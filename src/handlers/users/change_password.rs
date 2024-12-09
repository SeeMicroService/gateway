use crate::application;
use crate::domain::dto::Credentials;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn change_password(
    State(state): State<application::State>,
    Json(credentials): Json<Credentials>,
) -> (StatusCode, Json<Value>) {
    match state.users.change_password(&credentials).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({}))),
        Err(error) => {
            let error_message = Json(json!({"error": error.to_string()}));
            handle_error(error, error_message)
        }
    }
}
