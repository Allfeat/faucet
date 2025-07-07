use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full max-w-7xl px-6 mt-12 mb-4 flex justify-between items-center text-sm text-neutral-950">
            <div class="flex flex-col sm:flex-row gap-1 sm:gap-4">
                <span>"Allfeat labs. © 2025"</span>
                <span>"|"</span>
                <a
                    href="https://github.com/allfeat/allfeat-faucet"
                    target="_blank"
                    class="hover:text-teal-600 transition"
                >
                    Have an issue?
                </a>
            </div>
            <div class="flex items-center space-x-8">
                <a
                    href="https://github.com/allfeat"
                    target="_blank"
                    class="hover:text-teal-600 transition"
                >
                    <i class="fa-brands fa-github text-lg"></i>
                </a>
                <a
                    href="https://x.com/allfeat_IP"
                    target="_blank"
                    class="hover:text-teal-600 transition"
                >
                    <i class="fa-brands fa-x-twitter text-lg"></i>
                </a>
                <a
                    href="discord.allfeat.com"
                    target="_blank"
                    class="hover:text-teal-600 transition"
                >
                    <i class="fa-brands fa-discord text-lg"></i>
                </a>
            </div>
        </footer>
    }
}
