use crate::application::services::error::map_status;
use crate::domain::dto::{Credentials, Filenames, UserId};
use crate::domain::interfaces;
use crate::domain::models::User;
use axum::async_trait;
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::json;
use std::io;
use uuid::Uuid;

pub struct Users {
    client: Client,
    endpoint: String,
}

impl Users {
    pub fn new(client: Client, endpoint: String) -> Self {
        Users { client, endpoint }
    }
    async fn obtain_data<D>(response: Result<Response, reqwest::Error>) -> io::Result<D>
    where
        D: for<'de> Deserialize<'de>,
    {
        println!("{response:#?}");
        match response {
            Ok(response) => {
                map_status(response.status())?;
                match response.json::<D>().await {
                    Ok(user) => Ok(user),
                    Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", error))),
                }
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }

    fn parse_response(response: Result<Response, reqwest::Error>) -> io::Result<()> {
        match response {
            Ok(response) => {
                map_status(response.status())?;
                Ok(())
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", error))),
        }
    }
}

#[async_trait]
impl interfaces::Users for Users {
    type Error = io::Error;

    async fn create(&self, credentials: &Credentials) -> Result<(), Self::Error> {
        let response = self
            .client
            .post(format!("{}/add", self.endpoint))
            .json(credentials)
            .send()
            .await;
        Self::parse_response(response)
    }

    async fn get(&self, credentials: &Credentials) -> Result<User, Self::Error> {
        let response = self
            .client
            .get(format!("{}/info", self.endpoint))
            .json(credentials)
            .send()
            .await;
        Self::obtain_data(response).await
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error> {
        let response = self
            .client
            .delete(format!("{}/remove", self.endpoint))
            .json(&json!({"id": id}))
            .send()
            .await;
        Self::parse_response(response)
    }

    async fn change_password(&self, credentials: &Credentials) -> Result<(), Self::Error> {
        let response = self
            .client
            .patch(format!("{}/change_password", self.endpoint))
            .json(credentials)
            .send()
            .await;
        Self::parse_response(response)
    }

    async fn attach_file(&self, id: &Uuid, filename: &str) -> Result<(), Self::Error> {
        let response = self
            .client
            .post(format!("{}/attach_file", self.endpoint))
            .json(&json!({"id": id, "filename": filename}))
            .send()
            .await;
        Self::parse_response(response)
    }

    async fn detach_file(&self, id: &Uuid, filename: &str) -> Result<(), Self::Error> {
        let response = self
            .client
            .delete(format!("{}/detach_file", self.endpoint))
            .json(&json!({"id": id, "filename": filename}))
            .send()
            .await;
        Self::parse_response(response)
    }

    async fn get_filenames(&self, user_id: &UserId) -> Result<Vec<String>, Self::Error> {
        let response = self
            .client
            .get(format!("{}/get_filenames", self.endpoint))
            .json(user_id)
            .send()
            .await;
        match response {
            Ok(response) => {
                map_status(response.status())?;
                match response.json::<Filenames>().await {
                    Ok(filenames) => Ok(filenames.filenames),
                    Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
                }
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }
}
