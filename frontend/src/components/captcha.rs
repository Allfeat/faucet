use std::sync::OnceLock;

use gloo_net::http::Request;
use leptos::leptos_dom::logging;
use leptos::web_sys::js_sys::Reflect;
use leptos::{logging::log, prelude::*};
use wasm_bindgen::prelude::*;

static CAPTCHA_TOKEN_SIGNAL: OnceLock<WriteSignal<String>> = OnceLock::new();

#[component]
pub fn Captcha(setter: WriteSignal<String>) -> impl IntoView {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        _ = CAPTCHA_TOKEN_SIGNAL.set(setter);

        let closure = Closure::wrap(Box::new(move |token: JsValue| {
            if let Some(signal) = CAPTCHA_TOKEN_SIGNAL.get() {
                if let Some(t) = token.as_string() {
                    log!("✅ Captcha token received: {}", t);
                    signal.set(t);
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        // Set window["javascript_callback"] = closure
        Reflect::set(
            &JsValue::from(window()),
            &JsValue::from_str("cf_callback"),
            closure.as_ref().unchecked_ref(),
        )
        .expect("failed to set cf_callback on window");

        closure.forget();
    });

    let sitekey = LocalResource::new(move || async {
        match Request::get("/api/cf_sitekey").send().await {
            Ok(resp) => {
                let key = resp.text().await;
                logging::console_log(&format!("Received CF_SITEKEY: {key:?}"));
                key.unwrap_or_default()
            }
            Err(e) => {
                logging::console_error(&format!("Couldn't fetch CF_SITEKEY from API: {e}"));
                "".to_string()
            }
        }
    });

    view! {
        {move || match sitekey.get() {
        Some(x) => view! {
        <script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script>
        <div
            class="cf-turnstile mx-auto scale-90 flex justify-center"
            data-sitekey=x
            data-callback="cf_callback"
            data-theme="light"
        ></div>
        }
        .into_any(),
        None => ().into_any(),
    }
    }
    }
}
