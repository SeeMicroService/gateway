use crate::application;
use crate::domain::dto::Credentials;
use crate::handlers::auth::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

pub async fn register(
    State(state): State<application::State>,
    Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
    match credentials {
        Credentials {
            login: Some(_),
            password: Some(_),
            role: Some(_),
            ..
        } => {
            let user = state.users.create(&credentials).await;
            match user {
                Ok(_) => (StatusCode::OK, Json(Value::default())),
                Err(error) => handle_error(error),
            }
        }
        _ => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Invalid credentials"})),
        ),
    }
}
