use allfeat_faucet_shared::TransferStatus;
use leptos::prelude::*;

#[component]
pub fn ModalTx(show: RwSignal<bool>) -> impl IntoView {
    let ws_status = use_context::<RwSignal<Option<TransferStatus>>>()
        .expect("ws_status is provided in parent.");

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
                <div class="bg-white/90 backdrop-blur-md border border-gray-200 rounded-2xl shadow-2xl p-8 w-full max-w-md flex flex-col items-center gap-6 transition-all">
                    <h2 class="text-2xl font-bold text-gray-800 text-center">Transaction Status</h2>
                    <TransferState status=RwSignal::new(ws_status.get()).read_only() />
                </div>
            </div>
        </Show>
    }
}

#[component]
fn TransferState(status: ReadSignal<Option<TransferStatus>>) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center space-y-3">
            {move || {
                match status.get() {
                    Some(TransferStatus::TxInBlock { tx_hash }) => {
                        view! {
                            <div class="flex flex-col items-center text-center space-y-3">
                                <p class="text-emerald-600 font-semibold text-xl flex items-center gap-2">
                                    "✅ Transfer success!"
                                </p>
                                <p class="text-gray-600 text-sm break-all text-center">
                                    "The account just received some $MEL !"<br /> "Tx hash: "
                                    {tx_hash}
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    Some(other) => {

                        view! {
                            <div class="flex flex-col items-center text-center space-y-3">
                                <Spinner />
                                <p class="text-gray-600 text-base text-center max-w-sm">
                                    {format!("{other}")}
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    None => {

                        view! {
                            <div class="flex flex-col items-center text-center space-y-3">
                                <Spinner />
                                <p class="text-gray-600 text-base text-center max-w-sm">
                                    "Please wait..."
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                }
            }}

        </div>
    }
}

#[component]
fn Spinner() -> impl IntoView {
    view! {
        <div class="w-12 h-12 border-4 border-t-transparent border-emerald-400 rounded-full animate-spin" />
    }
}
