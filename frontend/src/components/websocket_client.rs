use allfeat_faucet_shared::TransferStatus;
use leptos::web_sys::MessageEvent;
use leptos::web_sys::WebSocket;
use leptos::{logging, prelude::*};
use std::sync::Arc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn WebSocketClient(client_id: Arc<str>) -> impl IntoView {
    let ws_status = use_context::<RwSignal<Option<TransferStatus>>>()
        .expect("ws_status is provided in parent.");

    Effect::new(move |_| {
        logging::log!("Initializing WS connection with client ID: {}", client_id);

        let ws_url = format!("/ws?client_id={client_id}");
        let ws = WebSocket::new(&ws_url).expect("failed to open WebSocket");

        logging::log!("Connected to WS backend.");

        let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                logging::log!("received new WS message: {}", text);

                match serde_json::from_str::<TransferStatus>(&text) {
                    Ok(status) => {
                        logging::log!("✅ Parsed status: {:?}", status);
                        ws_status.set(Some(status));
                    }
                    Err(e) => {
                        logging::log!("❌ Failed to parse status: {e}");
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
    });

    ().into_view()
}
