mod state;
mod events;
mod network;

use tokio::sync::broadcast;
use crate::events::Event;
use crate::state::SharedState;
use crate::network::start_server;

#[tokio::main]
async fn main() {
    let state = SharedState::new();
    let (tx, _) = broadcast::channel::<Event>(100);
    start_server(state, tx).await;
}
