use crate::application;
use crate::domain::dto::Credentials;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub async fn delete_user(
    State(state): State<application::State>,
    Json(credentials): Json<Credentials>,
) -> Response {
    if let Some(ref id) = credentials.id {
        match state.users.delete(id).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({"message": "User deleted successfully"})),
            )
                .into_response(),
            Err(error) => {
                let error_message = Json(json!({"error": "Failed to delete user"}));
                handle_error(error, error_message).into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User id is required" })),
        )
            .into_response()
    }
}
