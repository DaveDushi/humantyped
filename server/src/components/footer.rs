use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-neutral-800/50 mt-auto">
            <div class="max-w-6xl mx-auto px-6 py-12">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    // Brand
                    <div>
                        <div class="flex items-center gap-2.5 mb-4">
                            <div class="w-7 h-7 rounded-md bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-xs font-bold text-neutral-950">
                                "H"
                            </div>
                            <span class="text-base font-semibold">
                                "Human"<span class="text-emerald-400">"Typed"</span>
                            </span>
                        </div>
                        <p class="text-sm text-neutral-500 leading-relaxed">
                            "Proving humans still write on the internet. One keystroke at a time."
                        </p>
                    </div>

                    // Links
                    <div>
                        <h4 class="text-sm font-semibold text-neutral-300 mb-3">"Product"</h4>
                        <ul class="space-y-2">
                            <li><a href="/#how-it-works" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"How it works"</a></li>
                            <li><a href="/#privacy" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"Privacy"</a></li>
                            <li><a href="#install" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"Install Extension"</a></li>
                        </ul>
                    </div>

                    // Open Source
                    <div>
                        <h4 class="text-sm font-semibold text-neutral-300 mb-3">"Open Source"</h4>
                        <ul class="space-y-2">
                            <li><a href="https://github.com/humantyped" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"GitHub"</a></li>
                            <li><a href="/api/verify/example" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"API"</a></li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-neutral-800/50 mt-8 pt-8 flex items-center justify-between">
                    <p class="text-xs text-neutral-600">"No keystrokes stored. No identity tracked. Just proof of humanity."</p>
                    <p class="text-xs text-neutral-600">"MIT License"</p>
                </div>
            </div>
        </footer>
    }
}
