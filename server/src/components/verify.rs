use leptos::prelude::*;

use crate::db::TokenRecord;

#[component]
pub fn VerifyPage(record: Option<TokenRecord>) -> impl IntoView {
    view! {
        <main class="flex-1 flex items-center justify-center px-6 py-16">
            {match record {
                Some(r) => view! {
                    <div class="w-full max-w-lg">
                        <VerifiedCard record=r />
                    </div>
                }.into_any(),
                None => view! {
                    <div class="w-full max-w-lg">
                        <NotFoundCard />
                    </div>
                }.into_any(),
            }}
        </main>
    }
}

#[component]
fn VerifiedCard(record: TokenRecord) -> impl IntoView {
    let duration_secs = record.session_duration_ms as f64 / 1000.0;
    let duration_display = if duration_secs >= 60.0 {
        format!("{:.1} minutes", duration_secs / 60.0)
    } else {
        format!("{:.0} seconds", duration_secs)
    };
    let corrections = (record.correction_rate * record.character_count as f64).round() as i64;
    let confidence = record.confidence_score;
    let confidence_color = if confidence >= 70.0 {
        "from-emerald-500 to-emerald-400"
    } else if confidence >= 40.0 {
        "from-yellow-500 to-yellow-400"
    } else {
        "from-red-500 to-red-400"
    };

    let (banner_bg, banner_border, title, title_color, icon_color, icon_bg, icon_border) = if confidence >= 70.0 {
        ("bg-emerald-500/10", "border-emerald-500/20", "Human Verified", "text-emerald-400", "text-emerald-400", "bg-emerald-500/10", "border-emerald-500/30")
    } else if confidence >= 40.0 {
        ("bg-yellow-500/10", "border-yellow-500/20", "Suspicious Typing Pattern", "text-yellow-400", "text-yellow-400", "bg-yellow-500/10", "border-yellow-500/30")
    } else {
        ("bg-red-500/10", "border-red-500/20", "Likely Not Human", "text-red-400", "text-red-400", "bg-red-500/10", "border-red-500/30")
    };

    let is_pass = confidence >= 70.0;
    let is_warning = confidence >= 40.0 && confidence < 70.0;

    let token = record.token.clone();
    let created_at = record.created_at.clone();

    view! {
        <div class="rounded-2xl bg-neutral-900 border border-neutral-800 overflow-hidden shadow-2xl">
            // Amber accent bar
            <div class="h-1 bg-gradient-to-r from-amber-400 to-orange-400"></div>

            // Status banner
            <div class={format!("{banner_bg} border-b {banner_border} px-8 py-6 flex items-center gap-4")}>
                <div class={format!("w-12 h-12 rounded-full {icon_bg} border-2 {icon_border} flex items-center justify-center flex-shrink-0")}>
                    {if is_pass {
                        view! {
                            <svg class={format!("w-6 h-6 {icon_color}")} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                            </svg>
                        }.into_any()
                    } else if is_warning {
                        view! {
                            <svg class={format!("w-6 h-6 {icon_color}")} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                        }.into_any()
                    } else {
                        view! {
                            <svg class={format!("w-6 h-6 {icon_color}")} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        }.into_any()
                    }}
                </div>
                <div>
                    <h1 class={format!("text-2xl font-bold {title_color}")}>{title}</h1>
                    <p class="text-xs text-neutral-500 font-mono tracking-wider mt-1">{token}</p>
                </div>
            </div>

            // Stats
            <div class="px-8 py-6 space-y-5">
                <p class="text-neutral-300 leading-relaxed">
                    "Typed over "
                    <span class="text-white font-semibold">{duration_display}</span>
                    " at an average of "
                    <span class="text-white font-semibold">{format!("{:.0}", record.wpm_average)}" WPM"</span>
                    " with "
                    <span class="text-white font-semibold">{corrections}" corrections"</span>
                    "."
                </p>

                // Metrics grid
                <div class="grid grid-cols-2 gap-4">
                    <MetricCard label="AVG WPM" value=format!("{:.0}", record.wpm_average) />
                    <MetricCard label="WPM VARIANCE" value=format!("{:.1}", record.wpm_variance) />
                    <MetricCard label="CORRECTION RATE" value=format!("{:.1}%", record.correction_rate * 100.0) />
                    <MetricCard label="CHARACTERS" value=record.character_count.to_string() />
                </div>

                // Confidence meter
                <div>
                    <div class="flex items-center justify-between text-xs text-neutral-500 mb-2">
                        <span class="uppercase tracking-wider font-medium">"Human Confidence"</span>
                        <span class="text-white font-semibold">{format!("{:.0}%", confidence)}</span>
                    </div>
                    <div class="h-3 rounded-full bg-neutral-800 overflow-hidden">
                        <div
                            class={format!("h-full rounded-full bg-gradient-to-r {confidence_color} transition-all")}
                            style={format!("width: {}%", confidence.min(100.0))}
                        ></div>
                    </div>
                </div>

                // Timestamp
                <div class="flex items-center gap-2 text-xs text-neutral-500 pt-2 border-t border-neutral-800/50">
                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>"Certified: "{created_at}</span>
                </div>
            </div>

            // Explanation + CTA
            <div class="px-8 py-6 bg-neutral-900/50 border-t border-neutral-800/50">
                <h3 class="text-sm font-semibold text-neutral-300 mb-2">"What is HumanTyped?"</h3>
                <p class="text-xs text-neutral-500 leading-relaxed mb-4">
                    "HumanTyped is a Chrome extension that certifies social media posts were genuinely typed by a human. "
                    "It analyzes keystroke timing patterns — not the content — to generate a unique verification token. "
                    "No identity is tracked, no keystrokes are stored. Just proof of humanity."
                </p>
                <a
                    href="/#install"
                    class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-amber-400 hover:bg-amber-300 text-neutral-950 text-sm font-semibold transition-colors"
                >
                    "Get the Extension"
                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                    </svg>
                </a>
            </div>
        </div>
    }
}

#[component]
fn NotFoundCard() -> impl IntoView {
    view! {
        <div class="rounded-2xl bg-neutral-900 border border-neutral-800 overflow-hidden shadow-2xl">
            // Amber accent bar
            <div class="h-1 bg-gradient-to-r from-amber-400 to-orange-400"></div>

            <div class="text-center">
                <div class="bg-red-500/10 border-b border-red-500/20 px-8 py-6 flex items-center justify-center gap-4">
                    <div class="w-12 h-12 rounded-full bg-red-500/10 border-2 border-red-500/30 flex items-center justify-center">
                        <svg class="w-6 h-6 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </div>
                    <h1 class="text-2xl font-bold text-red-400">"Token Not Found"</h1>
                </div>
                <div class="px-8 py-8">
                    <p class="text-neutral-400 text-sm leading-relaxed mb-6">
                        "This verification token does not exist in our records. "
                        "It may be invalid, misspelled, or from a post that was not certified with HumanTyped."
                    </p>
                    <a
                        href="/"
                        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg border border-neutral-700 hover:border-neutral-600 text-neutral-300 text-sm font-medium transition-colors"
                    >
                        "Back to HumanTyped"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricCard(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="p-3 rounded-lg bg-neutral-800/50 border border-neutral-800/50">
            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium">{label}</p>
            <p class="text-lg font-bold text-white mt-1">{value}</p>
        </div>
    }
}
