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
                            <div class="w-7 h-7 rounded-md bg-gradient-to-br from-amber-400 to-orange-400 flex items-center justify-center text-xs font-bold text-neutral-950">
                                "H"
                            </div>
                            <span class="text-base font-semibold">
                                "Human"<span class="text-amber-400">"Typed"</span>
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
                            <li><a href="/#how-it-works" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"How It Works"</a></li>
                            <li><a href="/#privacy" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"Privacy"</a></li>
                            <li><a href="/#install" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"Install Extension"</a></li>
                            <li><a href="/feed" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"Verified Posts"</a></li>
                        </ul>
                    </div>

                    // Open Source
                    <div>
                        <h4 class="text-sm font-semibold text-neutral-300 mb-3">"Open Source"</h4>
                        <ul class="space-y-2">
                            <li>
                                <a
                                    href="https://github.com/DaveDushi/humantyped"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center gap-1.5 text-sm text-neutral-500 hover:text-neutral-300 transition-colors"
                                >
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                    </svg>
                                    "GitHub"
                                </a>
                            </li>
                            <li><a href="/api/verify/example" class="text-sm text-neutral-500 hover:text-neutral-300 transition-colors">"API"</a></li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-neutral-800/50 mt-8 pt-8 flex flex-col sm:flex-row items-center justify-between gap-2">
                    <p class="text-xs text-neutral-600">"No keystrokes stored. No identity tracked. Just proof of humanity."</p>
                    <p class="text-xs text-neutral-600">"MIT License"</p>
                </div>
            </div>
        </footer>
    }
}
