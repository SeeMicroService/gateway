use crate::application::{JwtAuth, Minio, State, Users};
use crate::handlers::{
    attach_file, change_password, delete_user, detach_file, get_file, get_filenames, login,
    put_file, refresh, register, validate_token,
};
use axum::http::{header, Method};
use axum::routing::{delete, get, patch, post, put};
use axum::{middleware, Router};
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod application;
mod domain;
mod handlers;

#[tokio::main]
async fn main() {
    if dotenvy::dotenv().is_err() {
        println!("Failed to initialize dotenvy. Using environment variables...");
    }
    tracing_subscriber::fmt::init();

    let auth_endpoint = env::var("AUTH_ENDPOINT").expect("AUTH_ENDPOINT must be set");
    let storage_endpoint = env::var("STORAGE_ENDPOINT").expect("STORAGE_ENDPOINT must be set");
    let users_endpoint =
        env::var("USERS_ENDPOINT").expect("Expected a USERS_ENDPOINT environment variable");
    let hostaddr = env::var("HOSTADDR").expect("HOSTADDR must be set");

    let client = reqwest::Client::new();

    let users = Arc::new(Users::new(client.clone(), users_endpoint));
    let storage = Arc::new(Minio::new(client.clone(), storage_endpoint));
    let auth = Arc::new(JwtAuth::new(client, auth_endpoint));

    let state = State {
        users,
        storage,
        auth,
    };

    let validate = middleware::from_fn_with_state(state.clone(), validate_token);

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_origin(Any)
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let files = Router::new()
        .route("/attach_file", post(attach_file))
        .route("/detach_file", delete(detach_file))
        .route("/put_file", put(put_file))
        .route("/get_file", get(get_file))
        .route("/get_filenames", get(get_filenames))
        .layer(validate.clone());

    let auth = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh).layer(validate.clone()));

    let users = Router::new()
        .route("/change_password", patch(change_password))
        .route("/delete_user", delete(delete_user))
        .layer(validate);

    let app = Router::new()
        .nest("/auth", auth)
        .nest("/files", files)
        .nest("/users", users);

    let api = Router::new()
        .nest("/api", app)
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(hostaddr).await.unwrap();

    axum::serve(listener, api).await.unwrap();
}
