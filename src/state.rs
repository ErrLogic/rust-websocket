use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ServerState {
    pub channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
