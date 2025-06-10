use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct Station {
    pub id: u32,
    pub name: String,
    pub reserved: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub stations: Arc<Mutex<Vec<Station>>>,
}
