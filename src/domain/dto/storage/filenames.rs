use serde::Deserialize;

#[derive(Deserialize)]
pub struct Filenames {
    pub filenames: Vec<String>,
}
