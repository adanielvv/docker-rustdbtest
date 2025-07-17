use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    EntityCreated { id: Uuid, name: String },
}

pub fn init_channel() -> (broadcast::Sender<Event>, broadcast::Receiver<Event>) {
    broadcast::channel(100)
}