use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenState {
    pub valid: bool,
}