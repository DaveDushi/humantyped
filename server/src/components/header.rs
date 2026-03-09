use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-neutral-800/50 backdrop-blur-sm sticky top-0 z-50 bg-neutral-950/80">
            <div class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
                <a href="/" class="flex items-center gap-2.5 group">
                    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-sm font-bold text-neutral-950 group-hover:scale-105 transition-transform">
                        "H"
                    </div>
                    <span class="text-lg font-semibold tracking-tight">
                        "Human"
                        <span class="text-emerald-400">"Typed"</span>
                    </span>
                </a>
                <nav class="flex items-center gap-6">
                    <a href="/feed" class="text-sm text-neutral-400 hover:text-neutral-100 transition-colors">"Verified Posts"</a>
                    <a href="/#how-it-works" class="text-sm text-neutral-400 hover:text-neutral-100 transition-colors">"How it works"</a>
                    <a href="/#privacy" class="text-sm text-neutral-400 hover:text-neutral-100 transition-colors">"Privacy"</a>
                    <a
                        href="#install"
                        class="text-sm font-medium px-4 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-neutral-950 transition-colors"
                    >
                        "Get Extension"
                    </a>
                </nav>
            </div>
        </header>
    }
}
