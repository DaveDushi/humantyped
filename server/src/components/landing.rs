use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <main class="flex-1">
            // ──────────────────────────────────────────
            // HERO — Split layout
            // ──────────────────────────────────────────
            <section class="relative overflow-hidden">
                <div class="max-w-6xl mx-auto px-6 pt-24 pb-16 lg:pt-32 lg:pb-24">
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
                        // Left — Text
                        <div class="text-center lg:text-left">
                            // Badge
                            <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-amber-500/20 bg-amber-500/5 text-amber-400 text-xs font-medium mb-8">
                                <span class="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse"></span>
                                "Free & Open Source"
                            </div>

                            <h1 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold tracking-tight leading-[1.08] mb-6">
                                "Prove Your Posts"
                                <br />
                                <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-400 to-orange-400">
                                    "Are Human"
                                </span>
                                <span class="inline-block w-0.5 h-10 md:h-14 bg-amber-400 ml-1 align-bottom animate-blink"></span>
                            </h1>

                            <p class="text-base md:text-lg text-neutral-400 max-w-xl mx-auto lg:mx-0 mb-10 leading-relaxed">
                                "HumanTyped analyzes your typing patterns — not your content — to certify social media posts were genuinely typed by a human. "
                                "Every certified post gets a verifiable token anyone can check."
                            </p>

                            <div class="flex flex-col sm:flex-row items-center justify-center lg:justify-start gap-4">
                                <a
                                    href="#install"
                                    class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-amber-400 hover:bg-amber-300 text-neutral-950 font-semibold text-base transition-all hover:shadow-lg hover:shadow-amber-500/25"
                                >
                                    "Get Started — It's Free"
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                    </svg>
                                </a>
                                <a
                                    href="https://github.com/DaveDushi/humantyped"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center gap-2 px-6 py-3 rounded-xl border border-neutral-700 hover:border-neutral-600 text-neutral-300 font-medium text-base transition-colors"
                                >
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                    </svg>
                                    "View on GitHub"
                                </a>
                            </div>
                        </div>

                        // Right — Verification card preview
                        <div class="relative flex justify-center lg:justify-end">
                            // Glow behind card
                            <div class="absolute -inset-4 bg-gradient-to-br from-amber-500/10 via-transparent to-emerald-500/5 rounded-3xl blur-2xl"></div>
                            <div class="relative w-full max-w-sm">
                                <div class="rounded-2xl bg-neutral-900 border border-neutral-800 overflow-hidden shadow-2xl">
                                    // Amber accent bar
                                    <div class="h-1 bg-gradient-to-r from-amber-400 to-orange-400"></div>

                                    // Status header
                                    <div class="px-6 py-5 flex items-center gap-4 border-b border-neutral-800/50">
                                        <div class="w-10 h-10 rounded-full bg-emerald-500/10 border-2 border-emerald-500/30 flex items-center justify-center flex-shrink-0">
                                            <svg class="w-5 h-5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                            </svg>
                                        </div>
                                        <div>
                                            <p class="text-base font-bold text-emerald-400">"Human Verified"</p>
                                            <p class="text-xs text-neutral-500 font-mono tracking-wider">"a7f3k9m2x1b4"</p>
                                        </div>
                                    </div>

                                    // Metrics
                                    <div class="px-6 py-5 grid grid-cols-2 gap-3">
                                        <div class="p-3 rounded-lg bg-neutral-800/30">
                                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium">"WPM"</p>
                                            <p class="text-xl font-bold text-white mt-1">"72"</p>
                                        </div>
                                        <div class="p-3 rounded-lg bg-neutral-800/30">
                                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium">"Variance"</p>
                                            <p class="text-xl font-bold text-white mt-1">"14.2"</p>
                                        </div>
                                        <div class="p-3 rounded-lg bg-neutral-800/30">
                                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium">"Corrections"</p>
                                            <p class="text-xl font-bold text-white mt-1">"8%"</p>
                                        </div>
                                        <div class="p-3 rounded-lg bg-neutral-800/30">
                                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium">"Confidence"</p>
                                            <p class="text-xl font-bold text-amber-400 mt-1">"94%"</p>
                                        </div>
                                    </div>

                                    // Confidence bar
                                    <div class="px-6 pb-5">
                                        <div class="h-2 rounded-full bg-neutral-800 overflow-hidden">
                                            <div class="h-full rounded-full bg-gradient-to-r from-emerald-500 to-emerald-400" style="width: 94%"></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // ──────────────────────────────────────────
            // WHY IT MATTERS
            // ──────────────────────────────────────────
            <section>
                // Gradient divider
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-3xl mx-auto px-6 py-20 text-center">
                    <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-6">
                        "AI Can Write. But It Can't "
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-400 to-orange-400">"Type."</span>
                    </h2>
                    <p class="text-base text-neutral-400 leading-relaxed max-w-2xl mx-auto">
                        "AI-generated text is flooding social media. Copy-pasting from ChatGPT is trivially easy and completely undetectable. "
                        "HumanTyped makes it detectable — by analyzing the one thing AI can't fake: the way your fingers move across the keyboard."
                    </p>
                </div>
            </section>

            // ──────────────────────────────────────────
            // HOW IT WORKS — Connected steps panel
            // ──────────────────────────────────────────
            <section id="how-it-works">
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-5xl mx-auto px-6 py-20">
                    <div class="text-center mb-12">
                        <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-4">"How It Works"</h2>
                        <p class="text-neutral-400 max-w-xl mx-auto">"Three steps. Two minutes. Zero data collected."</p>
                    </div>

                    <div class="rounded-2xl bg-neutral-900/30 border border-neutral-800/50 overflow-hidden">
                        <div class="grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x divide-neutral-800/50">
                            // Step 1
                            <div class="relative p-8 md:p-10">
                                <div class="absolute top-6 right-6 text-6xl font-bold text-neutral-800/40 select-none pointer-events-none">"1"</div>
                                <div class="relative">
                                    <div class="w-10 h-10 rounded-lg bg-amber-400/10 border border-amber-400/20 flex items-center justify-center mb-5">
                                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                        </svg>
                                    </div>
                                    <h3 class="text-base font-semibold mb-3">"Install the Extension"</h3>
                                    <p class="text-sm text-neutral-400 leading-relaxed">
                                        "Add HumanTyped to Chrome in 2 minutes. It runs silently in the background on X/Twitter — no setup, no account."
                                    </p>
                                </div>
                            </div>

                            // Step 2
                            <div class="relative p-8 md:p-10">
                                <div class="absolute top-6 right-6 text-6xl font-bold text-neutral-800/40 select-none pointer-events-none">"2"</div>
                                <div class="relative">
                                    <div class="w-10 h-10 rounded-lg bg-amber-400/10 border border-amber-400/20 flex items-center justify-center mb-5">
                                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                        </svg>
                                    </div>
                                    <h3 class="text-base font-semibold mb-3">"Type Your Post"</h3>
                                    <p class="text-sm text-neutral-400 leading-relaxed">
                                        "Write naturally on X/Twitter. We monitor timing between keys — never the keys themselves. Pasted content is detected and excluded."
                                    </p>
                                </div>
                            </div>

                            // Step 3
                            <div class="relative p-8 md:p-10">
                                <div class="absolute top-6 right-6 text-6xl font-bold text-neutral-800/40 select-none pointer-events-none">"3"</div>
                                <div class="relative">
                                    <div class="w-10 h-10 rounded-lg bg-amber-400/10 border border-amber-400/20 flex items-center justify-center mb-5">
                                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                                        </svg>
                                    </div>
                                    <h3 class="text-base font-semibold mb-3">"Anyone Can Verify"</h3>
                                    <p class="text-sm text-neutral-400 leading-relaxed">
                                        "A unique token is appended to your post. Anyone can click it to see your typing analysis and human confidence score."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // ──────────────────────────────────────────
            // EXAMPLE VERIFICATION — Bento metrics
            // ──────────────────────────────────────────
            <section>
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-4xl mx-auto px-6 py-16">
                    <div class="text-center mb-10">
                        <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-3">"Example Verification"</h2>
                        <p class="text-neutral-400">"What anyone sees when they click your token."</p>
                    </div>

                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                        // Confidence — 2 columns
                        <div class="col-span-2 p-6 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Human Confidence"</p>
                            <p class="text-5xl font-bold text-amber-400">
                                "94"
                                <span class="text-2xl text-amber-400/60">"%"</span>
                            </p>
                            <div class="mt-4 h-2 rounded-full bg-neutral-800 overflow-hidden">
                                <div class="h-full rounded-full bg-gradient-to-r from-emerald-500 to-emerald-400" style="width: 94%"></div>
                            </div>
                        </div>

                        // WPM
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Avg WPM"</p>
                            <p class="text-3xl font-bold text-white">"72"</p>
                        </div>

                        // Variance
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"WPM Variance"</p>
                            <p class="text-3xl font-bold text-white">"14.2"</p>
                        </div>

                        // Correction rate
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Corrections"</p>
                            <p class="text-3xl font-bold text-white">"8%"</p>
                        </div>

                        // Characters
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Characters"</p>
                            <p class="text-3xl font-bold text-white">"247"</p>
                        </div>

                        // Session
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Session"</p>
                            <p class="text-3xl font-bold text-white">"1.2m"</p>
                        </div>

                        // Token
                        <div class="p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50">
                            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mb-2">"Token"</p>
                            <p class="text-sm font-mono text-neutral-300 mt-2">"a7f3k9m2x1b4"</p>
                        </div>
                    </div>
                </div>
            </section>

            // ──────────────────────────────────────────
            // PRIVACY — Two-column comparison
            // ──────────────────────────────────────────
            <section id="privacy">
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-5xl mx-auto px-6 py-20">
                    <div class="text-center mb-12">
                        <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-4">"We Analyze Timing. Never Content."</h2>
                        <p class="text-neutral-400 max-w-xl mx-auto">"Built with a simple rule: never collect what you don't need."</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        // Left — What we analyze
                        <div class="rounded-2xl bg-neutral-900/30 border border-neutral-800/50 p-8">
                            <h3 class="text-base font-semibold text-emerald-400 mb-6 flex items-center gap-2">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                                </svg>
                                "What We Analyze"
                            </h3>
                            <ul class="space-y-4">
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-emerald-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Timing between keystrokes"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"Dwell and flight times only"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-emerald-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Typing speed variations"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"Average WPM and variance"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-emerald-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Correction patterns"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"Backspace rate and rhythm"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-emerald-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Session duration"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"How long you spent typing"</p>
                                    </div>
                                </li>
                            </ul>
                        </div>

                        // Right — What we never touch
                        <div class="rounded-2xl bg-neutral-900/30 border border-neutral-800/50 p-8">
                            <h3 class="text-base font-semibold text-red-400 mb-6 flex items-center gap-2">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
                                </svg>
                                "What We Never Touch"
                            </h3>
                            <ul class="space-y-4">
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-red-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"The keys you press"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"We never know which keys"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-red-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Your post content"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"Text never leaves your browser"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-red-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"Your username or identity"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"No accounts, no tracking"</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-4 h-4 text-red-400 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                    <div>
                                        <p class="text-sm font-medium text-neutral-200">"IP address or cookies"</p>
                                        <p class="text-xs text-neutral-500 mt-0.5">"Zero server-side tracking"</p>
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </div>

                    // Trust reinforcement
                    <div class="mt-8 text-center">
                        <p class="text-sm text-neutral-500 leading-relaxed">
                            "This is not a keylogger. We never record which keys you press — only the timing between them."
                        </p>
                        <a
                            href="https://github.com/DaveDushi/humantyped"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-1.5 text-sm text-amber-400 hover:text-amber-300 mt-2 transition-colors"
                        >
                            "Don't trust us? Read the code."
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                            </svg>
                        </a>
                    </div>
                </div>
            </section>

            // ──────────────────────────────────────────
            // INSTALL GUIDE
            // ──────────────────────────────────────────
            <section id="install">
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-3xl mx-auto px-6 py-20">
                    <div class="text-center mb-12">
                        <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-4">"Install in 2 Minutes"</h2>
                        <p class="text-neutral-400">"No Chrome Web Store needed. No account required."</p>
                    </div>

                    // Mobile warning
                    <div class="md:hidden mb-8 p-4 rounded-xl bg-amber-500/5 border border-amber-500/20 text-center">
                        <p class="text-sm text-amber-400">"Chrome extensions require a desktop browser. Bookmark this page and come back on your computer."</p>
                    </div>

                    // Steps
                    <div class="relative">
                        // Connecting line
                        <div class="absolute left-5 top-10 bottom-10 w-px bg-neutral-800 hidden sm:block"></div>

                        <div class="space-y-8">
                            // Step 1 — Download
                            <div class="relative flex gap-6">
                                <div class="flex-shrink-0 w-10 h-10 rounded-full bg-amber-400 text-neutral-950 font-bold text-sm flex items-center justify-center z-10">"1"</div>
                                <div class="flex-1 pb-2">
                                    <h3 class="text-base font-semibold mb-2">"Download the extension"</h3>
                                    <p class="text-sm text-neutral-400 mb-3">"Clone the repo or download the ZIP:"</p>
                                    <div class="rounded-lg bg-neutral-900 border border-neutral-800 p-4 font-mono text-sm text-neutral-300 overflow-x-auto">
                                        "git clone https://github.com/DaveDushi/humantyped.git"
                                    </div>
                                    <p class="text-sm text-neutral-500 mt-3">
                                        "No Git? "
                                        <a
                                            href="https://github.com/DaveDushi/humantyped/archive/refs/heads/main.zip"
                                            class="text-amber-400 hover:text-amber-300 underline underline-offset-2 transition-colors"
                                        >
                                            "Download ZIP instead"
                                        </a>
                                    </p>
                                </div>
                            </div>

                            // Step 2 — Open extensions
                            <div class="relative flex gap-6">
                                <div class="flex-shrink-0 w-10 h-10 rounded-full bg-amber-400 text-neutral-950 font-bold text-sm flex items-center justify-center z-10">"2"</div>
                                <div class="flex-1 pb-2">
                                    <h3 class="text-base font-semibold mb-2">"Open Chrome Extensions"</h3>
                                    <p class="text-sm text-neutral-400 mb-3">"Type this in your address bar:"</p>
                                    <div class="rounded-lg bg-neutral-900 border border-neutral-800 p-4 font-mono text-sm text-neutral-300">
                                        "chrome://extensions"
                                    </div>
                                    <p class="text-xs text-neutral-500 mt-2">"Or go to Menu > Extensions > Manage Extensions"</p>
                                </div>
                            </div>

                            // Step 3 — Enable & Load
                            <div class="relative flex gap-6">
                                <div class="flex-shrink-0 w-10 h-10 rounded-full bg-amber-400 text-neutral-950 font-bold text-sm flex items-center justify-center z-10">"3"</div>
                                <div class="flex-1 pb-2">
                                    <h3 class="text-base font-semibold mb-2">"Enable Developer Mode & Load"</h3>
                                    <p class="text-sm text-neutral-400">
                                        "Toggle "<span class="font-semibold text-neutral-200">"Developer Mode"</span>" on (top right), then click "
                                        <span class="font-semibold text-neutral-200">"Load Unpacked"</span>" and select the "
                                        <code class="px-1.5 py-0.5 rounded bg-neutral-800 text-amber-400 text-xs">"extension/"</code>
                                        " folder."
                                    </p>
                                </div>
                            </div>

                            // Step 4 — Done
                            <div class="relative flex gap-6">
                                <div class="flex-shrink-0 w-10 h-10 rounded-full bg-emerald-500 text-neutral-950 font-bold text-sm flex items-center justify-center z-10">
                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                </div>
                                <div class="flex-1">
                                    <h3 class="text-base font-semibold mb-2">"Start Posting!"</h3>
                                    <p class="text-sm text-neutral-400">
                                        "Go to "
                                        <a href="https://x.com" target="_blank" rel="noopener noreferrer" class="text-amber-400 hover:text-amber-300 transition-colors">"x.com"</a>
                                        " and compose a tweet. Look for the blue "
                                        <span class="font-semibold text-neutral-200">"H"</span>
                                        " badge. Type your post naturally — a verification link is appended automatically when you hit Post."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>

                    // FAQ Accordion
                    <div class="mt-12 space-y-3">
                        <h3 class="text-sm font-semibold text-neutral-300 mb-4">"Common questions"</h3>

                        <details class="group rounded-xl bg-neutral-900/30 border border-neutral-800/50">
                            <summary class="flex items-center justify-between cursor-pointer px-5 py-4 text-sm font-medium text-neutral-300 hover:text-neutral-100 transition-colors">
                                "Is Developer Mode safe?"
                                <svg class="w-4 h-4 text-neutral-500 transition-transform group-open:rotate-180" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
                                </svg>
                            </summary>
                            <div class="px-5 pb-4 text-sm text-neutral-400 leading-relaxed">
                                "Yes. Developer Mode is how all Chrome extensions are developed and tested. It's used by millions of developers daily and doesn't compromise your browser's security. You can disable it anytime."
                            </div>
                        </details>

                        <details class="group rounded-xl bg-neutral-900/30 border border-neutral-800/50">
                            <summary class="flex items-center justify-between cursor-pointer px-5 py-4 text-sm font-medium text-neutral-300 hover:text-neutral-100 transition-colors">
                                "How do I uninstall?"
                                <svg class="w-4 h-4 text-neutral-500 transition-transform group-open:rotate-180" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
                                </svg>
                            </summary>
                            <div class="px-5 pb-4 text-sm text-neutral-400 leading-relaxed">
                                "Go to chrome://extensions and click Remove on the HumanTyped card. That's it — completely gone in one click."
                            </div>
                        </details>

                        <details class="group rounded-xl bg-neutral-900/30 border border-neutral-800/50">
                            <summary class="flex items-center justify-between cursor-pointer px-5 py-4 text-sm font-medium text-neutral-300 hover:text-neutral-100 transition-colors">
                                "I don't have Git installed"
                                <svg class="w-4 h-4 text-neutral-500 transition-transform group-open:rotate-180" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
                                </svg>
                            </summary>
                            <div class="px-5 pb-4 text-sm text-neutral-400 leading-relaxed">
                                "No problem! Use the \"Download ZIP\" link in step 1. Extract the ZIP file, then select the extension/ folder inside when loading unpacked. No terminal needed."
                            </div>
                        </details>

                        <details class="group rounded-xl bg-neutral-900/30 border border-neutral-800/50">
                            <summary class="flex items-center justify-between cursor-pointer px-5 py-4 text-sm font-medium text-neutral-300 hover:text-neutral-100 transition-colors">
                                "Why isn't this on the Chrome Web Store?"
                                <svg class="w-4 h-4 text-neutral-500 transition-transform group-open:rotate-180" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
                                </svg>
                            </summary>
                            <div class="px-5 pb-4 text-sm text-neutral-400 leading-relaxed">
                                "We're working on it! In the meantime, installing from source means you can verify every line of code yourself. The entire project is open source."
                            </div>
                        </details>
                    </div>
                </div>
            </section>

            // ──────────────────────────────────────────
            // FINAL CTA
            // ──────────────────────────────────────────
            <section>
                <div class="w-48 h-px mx-auto bg-gradient-to-r from-transparent via-amber-500/30 to-transparent"></div>

                <div class="max-w-3xl mx-auto px-6 py-24 text-center">
                    <h2 class="text-2xl md:text-3xl font-bold tracking-tight mb-4">"Start Proving You're Human"</h2>
                    <p class="text-neutral-400 max-w-lg mx-auto mb-8">"Free, open source, and private. Install in 2 minutes."</p>
                    <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
                        <a
                            href="#install"
                            class="inline-flex items-center gap-2 px-8 py-4 rounded-xl bg-amber-400 hover:bg-amber-300 text-neutral-950 font-semibold text-lg transition-all hover:shadow-lg hover:shadow-amber-500/25"
                        >
                            "Install Now"
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </a>
                        <a
                            href="https://github.com/DaveDushi/humantyped"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-2 px-8 py-4 rounded-xl border border-neutral-700 hover:border-neutral-600 text-neutral-300 font-semibold text-lg transition-colors"
                        >
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                            </svg>
                            "Star on GitHub"
                        </a>
                    </div>
                </div>
            </section>
        </main>
    }
}
