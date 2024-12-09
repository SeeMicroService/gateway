use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::Role;

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub id: Option<Uuid>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}