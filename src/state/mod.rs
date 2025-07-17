use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NewEntity {
    pub name: String,
}

#[derive(Clone)]
pub struct SharedState {
    pub entities: Arc<Mutex<HashMap<Uuid, Entity>>>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            entities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
