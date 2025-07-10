use captcha::cf_sitekey;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::SystemTime};
use websocket::ws_handler;

use axum::{
    extract::ws::Message,
    routing::{get, post},
    Router,
};
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod api;
mod captcha;
mod chain;
mod websocket;

#[derive(Debug, Default, Clone)]
pub struct FaucetState {
    pub ws_clients: Clients,
    pub last_claims: LastClaims,
}

/// Mapping of client ID and the ws sender.
pub type Clients = Arc<RwLock<HashMap<ClientId, UnboundedSender<Message>>>>;
pub type ClientId = String;

/// Mapping of an Address to a time to enforce the cooldown between claims per address.
pub type LastClaims = Arc<RwLock<HashMap<String, SystemTime>>>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    let state = FaucetState::default();

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/transfer", post(api::handle_transfer))
        .route("/api/cf_sitekey", get(cf_sitekey))
        .fallback_service(ServeDir::new("./frontend/dist"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!(
        "🚀 Started faucet backend on ws://{}",
        listener.local_addr().unwrap()
    );
    tracing::info!("🖥️ Now serving the frontend client on /",);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
