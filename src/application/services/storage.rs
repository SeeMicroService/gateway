use crate::application::services::error::map_status;
use crate::domain::dto::{File, Path};
use crate::domain::interfaces::Storage;
use axum::async_trait;
use reqwest::Client;
use std::io;

pub struct Minio {
    client: Client,
    endpoint: String,
}

impl Minio {
    pub fn new(client: Client, endpoint: String) -> Self {
        Minio { client, endpoint }
    }
}

#[async_trait]
impl Storage for Minio {
    type Error = io::Error;

    async fn put(&self, file: &File) -> Result<(), Self::Error> {
        let response = self
            .client
            .put(format!("{}/put_file", self.endpoint))
            .json(file)
            .send()
            .await;
        match response {
            Ok(response) => map_status(response.status()),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }

    async fn get_file(&self, filename: &Path) -> Result<Vec<u8>, Self::Error> {
        let response = self
            .client
            .get(format!("{}/get_file", self.endpoint))
            .json(filename)
            .send()
            .await;
        match response {
            Ok(response) => {
                map_status(response.status())?;
                response
                    .bytes()
                    .await
                    .map(|bytes| bytes.to_vec())
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Internal server error"))
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }

    async fn delete(&self, filename: &Path) -> Result<(), Self::Error> {
        let response = self
            .client
            .delete(format!("{}/delete", self.endpoint))
            .json(filename)
            .send()
            .await;
        match response {
            Ok(response) => map_status(response.status()),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", error))),
        }
    }
}
