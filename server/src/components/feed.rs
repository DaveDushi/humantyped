use leptos::prelude::*;

use crate::db::TokenRecord;

#[component]
pub fn FeedPage(tokens: Vec<TokenRecord>, total: i64) -> impl IntoView {
    view! {
        <main class="flex-1">
            <div class="max-w-4xl mx-auto px-6 py-16">
                // Header
                <div class="mb-12">
                    <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-emerald-500/20 bg-emerald-500/5 text-emerald-400 text-xs font-medium mb-6">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        "Live feed"
                    </div>
                    <h1 class="text-3xl md:text-4xl font-bold mb-3">"Verified Human Posts"</h1>
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
                        let status_icon = if confidence >= 70.0 { "✅" }
                                          else if confidence >= 40.0 { "⚠\u{fe0f}" }
                                          else { "❌" };
                        let duration_s = t.session_duration_ms as f64 / 1000.0;
                        let duration = if duration_s >= 60.0 {
                            format!("{:.1}m", duration_s / 60.0)
                        } else {
                            format!("{:.0}s", duration_s)
                        };

                        view! {
                            <a
                                href={format!("/verify/{}", t.token)}
                                class="block p-5 rounded-xl bg-neutral-900/50 border border-neutral-800/50 hover:border-emerald-500/20 transition-all group"
                            >
                                <div class="flex items-center justify-between mb-3">
                                    <div class="flex items-center gap-3">
                                        <span class="text-lg">{status_icon}</span>
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
                                <p class="text-2xl mb-2">"🫥"</p>
                                <p class="text-neutral-500">"No verified posts yet. Be the first!"</p>
                                <a href="/#install" class="inline-block mt-4 px-5 py-2.5 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-neutral-950 text-sm font-semibold transition-colors">
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
            <p class="text-xs text-neutral-500 mt-1">{label}</p>
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
