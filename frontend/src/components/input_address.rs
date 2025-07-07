use leptos::prelude::*;
use leptos::web_sys::js_sys::RegExp;

#[component]
pub fn InputAddress(address: RwSignal<String>) -> impl IntoView {
    let raw_input = RwSignal::new(String::new());
    let is_valid = RwSignal::new(false);
    let re = RegExp::new(r"^[1-9A-HJ-NP-Za-km-z]{47,48}$", "");

    view! {
        <div class="flex flex-col w-full items-center space-y-3">
            <label class="text-lg font-semibold text-gray-700 text-center">
                Enter your account address
            </label>

            <input
                type="text"
                placeholder="5D4siWcA6mfsdgjD9aPqTmnYz3M8q9A5kGR..."
                class="w-full px-5 py-3 rounded-xl border border-gray-300 shadow-sm focus:outline-none focus:ring-2 focus:ring-teal-400 focus:border-teal-500 text-base transition duration-200"
                on:input=move |ev| {
                    let input = event_target_value(&ev);
                    raw_input.set(input.clone());
                    let valid = re.test(&input);
                    is_valid.set(valid);
                    if valid {
                        address.set(input);
                    }
                }
            />

            <Show when=move || !is_valid.get() && !raw_input.get().is_empty()>
                <p class="text-red-600 text-base text-center">Invalid SS58 address format.</p>
            </Show>
        </div>
    }
}
