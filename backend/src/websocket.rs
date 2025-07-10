use allfeat_faucet_shared::TransferStatus;
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, net::SocketAddr};
use tracing::warn;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Query, State, WebSocketUpgrade,
    },
    http::StatusCode,
};
use tokio::sync::mpsc;

use crate::{ClientId, Clients, FaucetState};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<FaucetState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    tracing::info!("🔌 WebSocket connection from {}", addr);

    let Some(client_id) = params.get("client_id").cloned() else {
        tracing::warn!("Missing client_id in query params");
        return Err(StatusCode::BAD_REQUEST);
    };
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, client_id, state.ws_clients)))
}

async fn handle_socket(socket: WebSocket, client_id: ClientId, clients: Clients) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Register the new connected client
    clients.write().await.insert(client_id.clone(), tx);
    tracing::info!("🔌 {} connected via WebSocket", client_id);

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let client_id_read = client_id.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::info!("📨 {} sent: {}", client_id_read, text);
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = write_task => {},
        _ = read_task => {},
    };

    // client cleaning
    clients.write().await.remove(&client_id);
    tracing::info!("❌ {} disconnected", client_id);
}

pub async fn notify_client(clients: &Clients, client_id: &str, payload: &TransferStatus) {
    let clients_guard = clients.read().await;
    if let Some(tx) = clients_guard.get(client_id) {
        match serde_json::to_string(payload) {
            Ok(json) => {
                if let Err(e) = tx.send(Message::Text(json.into())) {
                    warn!("Failed to send message to {}: {}", client_id, e);
                }
            }
            Err(e) => {
                warn!("Failed to serialize payload for {}: {}", client_id, e);
            }
        }
    } else {
        warn!("No WebSocket client found with id {}", client_id);
    }
}
