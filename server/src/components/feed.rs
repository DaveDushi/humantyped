use leptos::prelude::*;

use crate::db::TokenRecord;

#[component]
pub fn FeedPage(tokens: Vec<TokenRecord>, total: i64) -> impl IntoView {
    view! {
        <main class="flex-1">
            <div class="max-w-4xl mx-auto px-6 py-16">
                // Header
                <div class="mb-12">
                    <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-amber-500/20 bg-amber-500/5 text-amber-400 text-xs font-medium mb-6">
                        <span class="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse"></span>
                        "Live feed"
                    </div>
                    <h1 class="text-3xl md:text-4xl font-bold tracking-tight mb-3">"Verified Human Posts"</h1>
                    <p class="text-neutral-400">
                        {total}" posts certified as human-typed. Every entry below was verified through keystroke biometric analysis."
                    </p>
                </div>

                // Stats bar
                <div class="grid grid-cols-3 gap-4 mb-10">
                    <StatCard label="Total Certified" value=total.to_string() />
                    <StatCard label="Avg Confidence" value=format!("{:.0}%", avg_confidence(&tokens)) />
                    <StatCard label="Avg WPM" value=format!("{:.0}", avg_wpm(&tokens)) />
                </div>

                // Token list
                <div class="space-y-3">
                    {tokens.into_iter().map(|t| {
                        let confidence = t.confidence_score;
                        let color = if confidence >= 70.0 { "text-emerald-400" }
                                    else if confidence >= 40.0 { "text-yellow-400" }
                                    else { "text-red-400" };
                        let bar_color = if confidence >= 70.0 { "bg-emerald-500" }
                                        else if confidence >= 40.0 { "bg-yellow-500" }
                                        else { "bg-red-500" };
                        let is_pass = confidence >= 70.0;
                        let is_warning = confidence >= 40.0 && confidence < 70.0;
                        let duration_s = t.session_duration_ms as f64 / 1000.0;
                        let duration = if duration_s >= 60.0 {
                            format!("{:.1}m", duration_s / 60.0)
                        } else {
                            format!("{:.0}s", duration_s)
                        };

                        view! {
                            <a
                                href={format!("/verify/{}", t.token)}
                                class="block p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50 hover:border-amber-500/20 transition-all group"
                            >
                                <div class="flex items-center justify-between mb-3">
                                    <div class="flex items-center gap-3">
                                        {if is_pass {
                                            view! {
                                                <div class="w-7 h-7 rounded-full bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center flex-shrink-0">
                                                    <svg class="w-3.5 h-3.5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                                    </svg>
                                                </div>
                                            }.into_any()
                                        } else if is_warning {
                                            view! {
                                                <div class="w-7 h-7 rounded-full bg-yellow-500/10 border border-yellow-500/20 flex items-center justify-center flex-shrink-0">
                                                    <svg class="w-3.5 h-3.5 text-yellow-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                                    </svg>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="w-7 h-7 rounded-full bg-red-500/10 border border-red-500/20 flex items-center justify-center flex-shrink-0">
                                                    <svg class="w-3.5 h-3.5 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                    </svg>
                                                </div>
                                            }.into_any()
                                        }}
                                        <span class="font-mono text-sm text-neutral-300 group-hover:text-white transition-colors">
                                            {t.token.clone()}
                                        </span>
                                    </div>
                                    <span class="text-xs text-neutral-500">{t.created_at.clone()}</span>
                                </div>

                                <div class="flex items-center gap-6 text-xs text-neutral-500">
                                    <span>{format!("{:.0} WPM", t.wpm_average)}</span>
                                    <span>{format!("{} chars", t.character_count)}</span>
                                    <span>{format!("{:.1}% corrections", t.correction_rate * 100.0)}</span>
                                    <span>{duration}</span>
                                    <span class={format!("ml-auto font-semibold {color}")}>{format!("{:.0}%", confidence)}</span>
                                </div>

                                // Mini confidence bar
                                <div class="mt-3 h-1 rounded-full bg-neutral-800 overflow-hidden">
                                    <div
                                        class={format!("h-full rounded-full {bar_color}")}
                                        style={format!("width: {}%", confidence.min(100.0))}
                                    ></div>
                                </div>
                            </a>
                        }
                    }).collect::<Vec<_>>()}

                    {if total == 0 {
                        Some(view! {
                            <div class="text-center py-16">
                                <div class="w-12 h-12 rounded-full bg-neutral-800 flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-6 h-6 text-neutral-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                    </svg>
                                </div>
                                <p class="text-neutral-500">"No verified posts yet. Be the first!"</p>
                                <a href="/#install" class="inline-block mt-4 px-5 py-2.5 rounded-lg bg-amber-400 hover:bg-amber-300 text-neutral-950 text-sm font-semibold transition-colors">
                                    "Get the Extension"
                                </a>
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>
        </main>
    }
}

#[component]
fn StatCard(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="p-4 rounded-xl bg-neutral-900/50 border border-neutral-800/50 text-center">
            <p class="text-2xl font-bold text-white">{value}</p>
            <p class="text-xs text-neutral-500 uppercase tracking-wider font-medium mt-1">{label}</p>
        </div>
    }
}

fn avg_confidence(tokens: &[TokenRecord]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    tokens.iter().map(|t| t.confidence_score).sum::<f64>() / tokens.len() as f64
}

fn avg_wpm(tokens: &[TokenRecord]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    tokens.iter().map(|t| t.wpm_average).sum::<f64>() / tokens.len() as f64
}
