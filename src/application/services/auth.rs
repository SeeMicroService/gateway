use crate::application::services::error::map_status;
use crate::domain::dto::{TokenDto, TokenState};
use crate::domain::interfaces::Auth;
use crate::domain::models::User;
use axum::async_trait;
use reqwest::{Client, Response};
use serde_json::json;
use std::io;

pub struct JwtAuth {
    client: Client,
    endpoint: String,
}

impl JwtAuth {
    pub fn new(client: Client, endpoint: String) -> Self {
        JwtAuth { client, endpoint }
    }
    async fn obtain_token(response: Result<Response, reqwest::Error>) -> io::Result<String> {
        match response {
            Ok(response) => {
                map_status(response.status())?;
                match response.json::<TokenDto>().await {
                    Ok(json) => Ok(json.token),
                    Err(err) => Err(io::Error::new(io::ErrorKind::InvalidInput, err.to_string())),
                }
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }
}

#[async_trait]
impl Auth for JwtAuth {
    type Error = io::Error;

    async fn get_token(&self, user: &User) -> Result<String, Self::Error> {
        let response = self
            .client
            .post(format!("{}/generate", self.endpoint))
            .json(user)
            .send()
            .await;
        Self::obtain_token(response).await
    }

    async fn refresh(&self, token: &str) -> Result<String, Self::Error> {
        let response = self
            .client
            .post(format!("{}/refresh", self.endpoint))
            .json(&json!({ "token": token }))
            .send()
            .await;
        Self::obtain_token(response).await
    }

    async fn validate_token(&self, token: &str) -> Result<bool, Self::Error> {
        let response = self
            .client
            .post(format!("{}/validate", self.endpoint))
            .json(&json!({ "token": token }))
            .send()
            .await;
        match response {
            Ok(response) => {
                let state = response.json::<TokenState>().await;
                match state {
                    Ok(state) => Ok(state.valid),
                    Err(err) => Err(io::Error::new(io::ErrorKind::InvalidInput, err.to_string())),
                }
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }
}
