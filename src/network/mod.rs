use warp::Filter;
use crate::state::{SharedState, Entity};
use crate::events::Event;
use tokio::sync::broadcast::Sender;
use uuid::Uuid;
use std::convert::Infallible;
use warp::reply::html;

pub mod ws;
use crate::network::ws::ws_filter;

pub async fn start_server(state: SharedState, tx: Sender<Event>) {
    // Shared filters
    let state_filter = warp::any().map(move || state.clone());
    let tx_filter = warp::any().map(move || tx.clone());

    // POST /create
    let create_filter = warp::path("create")
        .and(warp::post())
        .and(state_filter.clone())
        .and(tx_filter.clone())
        .and(warp::body::json())
        .and_then(handle_create);

    // GET /entities
    let entities_filter = warp::path("entities")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(handle_get_entities);

    // WebSocket /ws
    let websocket_filter = ws_filter();

    // Static files and index
    let static_files = warp::fs::dir("static");

    // Combine all routes
    let routes = create_filter
        .or(entities_filter)
        .or(websocket_filter)
        .or(static_files);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

async fn handle_create(
    state: SharedState,
    tx: Sender<Event>,
    input: crate::state::NewEntity,
) -> Result<impl warp::Reply, Infallible> {
    let mut entities = state.entities.lock().unwrap();
    let id = Uuid::new_v4();
    let entity = Entity { id, name: input.name };

    entities.insert(id, entity.clone());

    let _ = tx.send(Event::EntityCreated { id, name: entity.name.clone() });

    Ok(warp::reply::json(&entity))
}

async fn handle_get_entities(
    state: SharedState,
) -> Result<impl warp::Reply, Infallible> {
    let entities = state.entities.lock().unwrap();
    let mut html_string = String::new();
    for e in entities.values() {
        html_string.push_str(&format!("<li>{} – {}</li>", e.id, e.name));
    }
    Ok(html(format!("<ul>{}</ul>", html_string)))
}