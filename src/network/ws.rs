// WebSocket server implementation using Warp and Tokio
use warp::Filter;
use warp::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use std::convert::Infallible;

pub fn ws_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_socket)
        })
}

async fn handle_socket(mut ws: WebSocket) {
    println!("WebSocket connected");

    while let Some(result) = ws.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    // Echo the message back
                    if let Err(e) = ws.send(msg).await {
                        eprintln!("WebSocket send error: {}", e);
                        break;
                    }
                }
            },
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("WebSocket disconnected");
}