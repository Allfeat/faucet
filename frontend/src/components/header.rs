use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full flex items-center px-6 pt-6">
            <img src="assets/logo.svg" alt="Allfeat Logo" class="w-16 h-16" />
        </header>
    }
}
