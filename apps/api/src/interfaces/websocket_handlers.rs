use std::sync::Arc;
use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::state::AppState;

pub async fn handle_ws(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, session_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, session_id: String) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to session broadcast
    let tx = state.get_or_create_sender(&session_id).await;
    let mut rx = tx.subscribe();

    // Send all existing events first (replay on connect)
    {
        let events = state.events.read().await;
        if let Some(existing) = events.get(&session_id) {
            for event in existing {
                if let Ok(json) = serde_json::to_string(event) {
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        return;
                    }
                }
            }
        }
    }

    // Forward new broadcast messages to WS client
    let send_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    });

    // Keep alive / handle pings from client (discard messages)
    while let Some(Ok(_msg)) = receiver.next().await {}

    send_task.abort();
}
