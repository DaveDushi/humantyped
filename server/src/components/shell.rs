use leptos::prelude::*;

#[component]
pub fn Shell(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content="Prove your social media posts were genuinely typed by a human. Keystroke biometric analysis with public verification. Free, open source, private." />
                <meta name="theme-color" content="#0a0a0a" />
                <meta property="og:title" content="HumanTyped — Prove Your Posts Are Human" />
                <meta property="og:description" content="Keystroke biometric verification for social media. Prove you typed it yourself." />
                <meta property="og:type" content="website" />
                <link rel="stylesheet" href="/styles.css" />
                <title>{title}</title>
            </head>
            <body class="bg-neutral-950 text-neutral-100 antialiased min-h-screen flex flex-col">
                {children()}
            </body>
        </html>
    }
}
