use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenDto {
    pub token: String,
}