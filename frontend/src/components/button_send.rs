use std::sync::Arc;

use allfeat_faucet_shared::TransferRequest;
use gloo_net::http::Request;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

#[component]
pub fn ButtonSend(
    address: ReadSignal<String>,
    client_id: Arc<str>,
    cf_token: ReadSignal<String>,
) -> impl IntoView {
    let has_requested = RwSignal::new(false);
    let disabled = Signal::derive(move || {
        address.get().is_empty() || cf_token.get().is_empty() || has_requested.get()
    });
    let show_tx_modal =
        use_context::<RwSignal<bool>>().expect("there to be a `show_tx_modal` signal provided");

    view! {
        <div class="flex justify-center mt-6">
            <button
                on:click=move |_| {
                    let client_id = client_id.clone();
                    let address = address.get();
                    let cf_token = cf_token.get();
                    spawn_local(async move {
                        has_requested.set(true);
                        request_transfer(address, client_id, cf_token, show_tx_modal.write_only())
                            .await;
                    });
                }

                disabled=disabled
                class="
                w-60 bg-teal-500 hover:bg-teal-600 text-white font-semibold
                text-lg py-3 px-6 rounded-xl transition-all duration-200
                disabled:opacity-50 disabled:cursor-not-allowed shadow-md
                "
            >
                "Send me $MEL"
            </button>
        </div>
    }
}

async fn request_transfer(
    address: String,
    client_id: Arc<str>,
    token: String,
    show_tx_modal: WriteSignal<bool>,
) {
    let api_url = "/api/transfer".to_string();
    let body = TransferRequest {
        address,
        client_id: client_id.to_string(),
        token,
    };
    let res = Request::post(&api_url)
        .header("Content-Type", "application/json")
        .json(&body)
        .unwrap()
        .send()
        .await;

    match res {
        Ok(resp) => {
            if resp.ok() {
                log!("✅ Transfer request sent successfully");
                show_tx_modal.set(true)
            } else {
                log!("❌ Server returned error status: {}", resp.status());
            }
        }
        Err(e) => {
            log!("💥 Failed to send request: {e}");
        }
    }
}
