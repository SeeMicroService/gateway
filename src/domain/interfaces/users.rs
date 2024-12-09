use crate::domain::dto::{Credentials, UserId};
use crate::domain::models::User;
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Users: Send + Sync {
    type Error;
    async fn create(&self, credentials: &Credentials) -> Result<(), Self::Error>;

    async fn get(&self, credentials: &Credentials) -> Result<User, Self::Error>;

    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error>;

    async fn change_password(&self, credentials: &Credentials) -> Result<(), Self::Error>;

    async fn attach_file(&self, id: &Uuid, filename: &str) -> Result<(), Self::Error>;

    async fn detach_file(&self, id: &Uuid, filename: &str) -> Result<(), Self::Error>;

    async fn get_filenames(&self, user_id: &UserId) -> Result<Vec<String>, Self::Error>;
}
