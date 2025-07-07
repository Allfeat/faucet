use allfeat_faucet_frontend::App;
use leptos::prelude::*;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // set up env variables files.
    dotenvy::dotenv().ok();

    mount_to_body(|| {
        view! { <App /> }
    })
}
