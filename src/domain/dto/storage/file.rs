use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct File {
    pub id: Option<Uuid>,
    pub name: String,
    pub content: Option<String>,
}
