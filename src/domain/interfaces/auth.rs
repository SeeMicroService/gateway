use crate::domain::models::User;
use axum::async_trait;

#[async_trait]
pub trait Auth: Send + Sync {
    type Error;

    async fn get_token(&self, user: &User) -> Result<String, Self::Error>;

    async fn refresh(&self, token: &str) -> Result<String, Self::Error>;

    async fn validate_token(&self, token: &str) -> Result<bool, Self::Error>;
}
