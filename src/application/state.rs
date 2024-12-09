use crate::domain::interfaces::{Auth, Storage, Users};
use std::io;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub users: Arc<dyn Users<Error = io::Error>>,
    pub storage: Arc<dyn Storage<Error = io::Error>>,
    pub auth: Arc<dyn Auth<Error = io::Error>>,
}
