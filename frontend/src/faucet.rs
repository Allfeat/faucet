use std::sync::Arc;

use crate::components::button_send::ButtonSend;
use crate::components::captcha::Captcha;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::input_address::InputAddress;
use crate::components::modal::ModalTx;
use crate::components::websocket_client::WebSocketClient;
use allfeat_faucet_shared::TransferStatus;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Faucet() -> impl IntoView {
    // The address input.
    let address = RwSignal::new(String::new());

    // If we may show the transaction status modal or not.
    let show_tx_modal = RwSignal::new(false);

    // The reponse token of the CF challenge.
    let cf_token = RwSignal::new(String::new());

    // The last status message received through websocket.
    let ws_status = RwSignal::<Option<TransferStatus>>::new(None);

    // The client ID used to communicate with the websocket backend.
    let client_id = generate_client_id();

    provide_context(show_tx_modal);
    provide_context(ws_status);

    view! {
        <div class="min-h-screen flex flex-col justify-between bg-gradient-to-br from-white via-[#f0f9ff] to-[#d1f3f0]">
            <Header />

            <main class="flex flex-col items-center px-4 space-y-12">
                // Titre
                <div class="text-center">
                    <div class="text-sm uppercase text-teal-600 font-semibold tracking-wide mb-1">
                        "MELODIE (TESTNET)"
                    </div>
                    <h1 class="text-5xl font-bold text-gray-800 tracking-tight">Allfeat Faucet</h1>
                </div>

                <div class="w-full max-w-xl bg-white/70 backdrop-blur-md shadow-xl rounded-2xl p-8 border border-gray-200 flex flex-col items-center space-y-6">
                    <InputAddress address />
                    <Captcha setter=cf_token.write_only() />
                    <ButtonSend
                        address=address.read_only()
                        client_id=client_id.clone()
                        cf_token=cf_token.read_only()
                    />
                </div>

                <div class="mt-8 max-w-xl text-sm text-gray-600 text-center leading-relaxed">
                    "This faucet provides test "
                    <span class="font-medium text-teal-600">"$MEL"</span>" tokens on the Allfeat "
                    <span class="font-medium">Melodie</span>" testnet. "
                    "Tokens are intended for development and testing purposes only and have no real-world value.\n
                    You may request once per address every 24 hours."
                </div>
            </main>

            <Footer />
            <ModalTx show=show_tx_modal />
            <WebSocketClient client_id />
        </div>
    }
}

fn generate_client_id() -> Arc<str> {
    let uuid = Uuid::new_v4().to_string();
    Arc::from(uuid)
}
