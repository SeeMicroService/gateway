use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Path {
    pub path: String,
}