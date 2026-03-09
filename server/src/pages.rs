use axum::extract::{Path, State};
use axum::response::Html;
use leptos::prelude::*;

use crate::components::feed::FeedPage;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::landing::LandingPage;
use crate::components::shell::Shell;
use crate::components::verify::VerifyPage;
use crate::db;
use crate::AppState;

pub async fn landing() -> Html<String> {
    let html = view! {
        <Shell title="HumanTyped — Prove You Typed It Yourself">
            <Header />
            <LandingPage />
            <Footer />
        </Shell>
    }
    .to_html();
    Html(html)
}

pub async fn feed(
    State(state): State<AppState>,
) -> Html<String> {
    let tokens = db::list_recent(&state.db, 50).await;
    let total = db::count_tokens(&state.db).await;

    let html = view! {
        <Shell title="Verified Posts — HumanTyped">
            <Header />
            <FeedPage tokens=tokens total=total />
            <Footer />
        </Shell>
    }
    .to_html();
    Html(html)
}

pub async fn verify_page(
    Path(token): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    let record = db::verify(&state.db, token).await;

    let html = view! {
        <Shell title="Verification — HumanTyped">
            <Header />
            <VerifyPage record=record />
            <Footer />
        </Shell>
    }
    .to_html();
    Html(html)
}
