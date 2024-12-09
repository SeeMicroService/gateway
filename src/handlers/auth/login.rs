use crate::application;
use crate::domain::dto::Credentials;
use crate::domain::models::User;
use crate::handlers::auth::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

async fn get_token(state: &application::State, user: &User) -> (StatusCode, Json<Value>) {
    let token = state.auth.get_token(user).await;
    match token {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({"user_id": user.id, "token": token})),
        ),
        Err(error) => handle_error(error),
    }
}

pub async fn login(
    State(state): State<application::State>,
    Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
    match credentials {
        Credentials {
            login: Some(_),
            password: Some(_),
            ..
        } => {
            let user = state.users.get(&credentials).await;
            match user {
                Ok(user) => get_token(&state, &user).await,
                Err(error) => handle_error(error),
            }
        }
        _ => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Invalid credentials"})),
        ),
    }
}
