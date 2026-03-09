use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <main class="flex-1">
            // Hero
            <section class="relative overflow-hidden">
                // Gradient orb background
                <div class="absolute inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute -top-40 -right-40 w-96 h-96 bg-emerald-500/10 rounded-full blur-3xl"></div>
                    <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-emerald-500/5 rounded-full blur-3xl"></div>
                </div>

                <div class="max-w-4xl mx-auto px-6 pt-24 pb-20 text-center relative">
                    // Badge
                    <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-emerald-500/20 bg-emerald-500/5 text-emerald-400 text-xs font-medium mb-8">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        "Keystroke biometric verification"
                    </div>

                    <h1 class="text-5xl md:text-7xl font-bold tracking-tight leading-[1.1] mb-6">
                        "Prove you"
                        <br />
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-400">
                            "typed it yourself"
                        </span>
                    </h1>

                    <p class="text-lg md:text-xl text-neutral-400 max-w-2xl mx-auto mb-10 leading-relaxed">
                        "HumanTyped analyzes your keystroke patterns to certify that your social media posts were genuinely typed by a human. "
                        "Every certified post gets a verifiable token anyone can check."
                    </p>

                    <div class="flex items-center justify-center gap-4">
                        <a
                            href="#install"
                            class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-neutral-950 font-semibold text-base transition-all hover:shadow-lg hover:shadow-emerald-500/25"
                        >
                            "Install for Chrome"
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </a>
                        <a
                            href="/#how-it-works"
                            class="inline-flex items-center gap-2 px-6 py-3 rounded-xl border border-neutral-700 hover:border-neutral-600 text-neutral-300 font-medium text-base transition-colors"
                        >
                            "Learn more"
                        </a>
                    </div>

                    // Example token preview
                    <div class="mt-16 inline-flex items-center gap-3 px-5 py-3 rounded-2xl bg-neutral-900 border border-neutral-800 shadow-2xl">
                        <span class="text-2xl">"✅"</span>
                        <div class="text-left">
                            <p class="text-sm text-neutral-300 font-medium">"This post is human-certified"</p>
                            <p class="text-xs text-neutral-500 font-mono">"[HumanTyped: a7f3k9m2x1b4]"</p>
                        </div>
                    </div>
                </div>
            </section>

            // How it works
            <section id="how-it-works" class="border-t border-neutral-800/50">
                <div class="max-w-6xl mx-auto px-6 py-24">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl md:text-4xl font-bold mb-4">"How it works"</h2>
                        <p class="text-neutral-400 max-w-xl mx-auto">"Three steps from typing to certification. No accounts, no identity, no stored keystrokes."</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Step 1
                        <div class="group relative p-8 rounded-2xl bg-neutral-900/50 border border-neutral-800/50 hover:border-emerald-500/20 transition-all">
                            <div class="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-400 font-bold text-sm mb-5">
                                "1"
                            </div>
                            <h3 class="text-lg font-semibold mb-3">"Type naturally"</h3>
                            <p class="text-sm text-neutral-400 leading-relaxed">
                                "Write your post as usual. The extension silently monitors keystroke timing, rhythm, and corrections — "
                                "never the actual content. Paste is blocked to ensure authentic typing."
                            </p>
                        </div>

                        // Step 2
                        <div class="group relative p-8 rounded-2xl bg-neutral-900/50 border border-neutral-800/50 hover:border-emerald-500/20 transition-all">
                            <div class="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-400 font-bold text-sm mb-5">
                                "2"
                            </div>
                            <h3 class="text-lg font-semibold mb-3">"Get certified"</h3>
                            <p class="text-sm text-neutral-400 leading-relaxed">
                                "When you hit post, a unique 12-character token is generated from your typing biometrics. "
                                "Only anonymized timing metadata is sent to the server — zero personal data."
                            </p>
                        </div>

                        // Step 3
                        <div class="group relative p-8 rounded-2xl bg-neutral-900/50 border border-neutral-800/50 hover:border-emerald-500/20 transition-all">
                            <div class="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-400 font-bold text-sm mb-5">
                                "3"
                            </div>
                            <h3 class="text-lg font-semibold mb-3">"Anyone can verify"</h3>
                            <p class="text-sm text-neutral-400 leading-relaxed">
                                "The token is appended to your post. Anyone can click it to see a public verification page showing "
                                "typing speed, correction rate, and a human confidence score."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Stats preview
            <section class="border-t border-neutral-800/50">
                <div class="max-w-4xl mx-auto px-6 py-24">
                    <div class="rounded-2xl bg-neutral-900/50 border border-neutral-800/50 overflow-hidden">
                        <div class="p-8 md:p-12">
                            <h2 class="text-2xl md:text-3xl font-bold mb-2">"What gets verified"</h2>
                            <p class="text-neutral-400 mb-8">"Anonymized metrics that prove human typing patterns."</p>

                            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                                <div>
                                    <p class="text-3xl font-bold text-emerald-400">"72"</p>
                                    <p class="text-sm text-neutral-500 mt-1">"Words per minute"</p>
                                </div>
                                <div>
                                    <p class="text-3xl font-bold text-emerald-400">"14.2"</p>
                                    <p class="text-sm text-neutral-500 mt-1">"WPM variance"</p>
                                </div>
                                <div>
                                    <p class="text-3xl font-bold text-emerald-400">"8%"</p>
                                    <p class="text-sm text-neutral-500 mt-1">"Correction rate"</p>
                                </div>
                                <div>
                                    <p class="text-3xl font-bold text-emerald-400">"94"</p>
                                    <p class="text-sm text-neutral-500 mt-1">"Confidence score"</p>
                                </div>
                            </div>
                        </div>

                        // Confidence bar
                        <div class="px-8 md:px-12 pb-8 md:pb-12">
                            <div class="flex items-center justify-between text-xs text-neutral-500 mb-2">
                                <span>"Human confidence"</span>
                                <span>"94%"</span>
                            </div>
                            <div class="h-2.5 rounded-full bg-neutral-800 overflow-hidden">
                                <div class="h-full rounded-full bg-gradient-to-r from-emerald-500 to-emerald-400" style="width: 94%"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Privacy
            <section id="privacy" class="border-t border-neutral-800/50">
                <div class="max-w-4xl mx-auto px-6 py-24">
                    <div class="text-center mb-12">
                        <h2 class="text-3xl md:text-4xl font-bold mb-4">"Privacy first. Always."</h2>
                        <p class="text-neutral-400 max-w-xl mx-auto">"We built HumanTyped with a simple rule: never collect what you don't need."</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="flex items-start gap-4 p-6 rounded-xl bg-neutral-900/30 border border-neutral-800/30">
                            <span class="text-emerald-400 text-lg mt-0.5">"✓"</span>
                            <div>
                                <p class="font-medium text-sm">"100% client-side analysis"</p>
                                <p class="text-xs text-neutral-500 mt-1">"All keystroke analysis happens in your browser. Nothing leaves your machine except anonymized timing data."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-4 p-6 rounded-xl bg-neutral-900/30 border border-neutral-800/30">
                            <span class="text-emerald-400 text-lg mt-0.5">"✓"</span>
                            <div>
                                <p class="font-medium text-sm">"No keystrokes stored"</p>
                                <p class="text-xs text-neutral-500 mt-1">"We never see what you type. Only timing patterns between keys, not the keys themselves."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-4 p-6 rounded-xl bg-neutral-900/30 border border-neutral-800/30">
                            <span class="text-emerald-400 text-lg mt-0.5">"✓"</span>
                            <div>
                                <p class="font-medium text-sm">"No identity tracking"</p>
                                <p class="text-xs text-neutral-500 mt-1">"No IP logging, no cookies, no accounts. Tokens are cryptographically derived from session data."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-4 p-6 rounded-xl bg-neutral-900/30 border border-neutral-800/30">
                            <span class="text-emerald-400 text-lg mt-0.5">"✓"</span>
                            <div>
                                <p class="font-medium text-sm">"Open source"</p>
                                <p class="text-xs text-neutral-500 mt-1">"Every line of code is public. Audit the extension, audit the server. Trust through transparency."</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // CTA
            <section id="install" class="border-t border-neutral-800/50">
                <div class="max-w-4xl mx-auto px-6 py-24 text-center">
                    <h2 class="text-3xl md:text-4xl font-bold mb-4">"Start certifying your posts"</h2>
                    <p class="text-neutral-400 max-w-lg mx-auto mb-8">"Install the Chrome extension and prove your posts are human-written. Free, open source, and private."</p>
                    <a
                        href="#"
                        class="inline-flex items-center gap-2 px-8 py-4 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-neutral-950 font-semibold text-lg transition-all hover:shadow-lg hover:shadow-emerald-500/25"
                    >
                        "Add to Chrome"
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                        </svg>
                    </a>
                </div>
            </section>
        </main>
    }
}
