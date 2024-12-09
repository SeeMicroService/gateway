use crate::domain::dto::{File, Path};
use axum::async_trait;

#[async_trait]
pub trait Storage: Send + Sync {
    type Error;
    async fn put(&self, file: &File) -> Result<(), Self::Error>;

    async fn get_file(&self, path: &Path) -> Result<Vec<u8>, Self::Error>;

    async fn delete(&self, path: &Path) -> Result<(), Self::Error>;
}
