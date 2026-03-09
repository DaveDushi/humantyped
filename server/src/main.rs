mod api;
mod components;
mod db;
mod pages;
mod rate_limit;

use axum::routing::{get, post};
use axum::Router;
use rate_limit::RateLimiter;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_rusqlite::Connection;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: Connection,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "humantyped.db".to_string());

    let db = db::init_db(&db_path).await;

    let state = AppState {
        db,
        rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
    };

    // Spawn rate limiter cleanup task
    let limiter = state.rate_limiter.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            limiter.lock().unwrap().cleanup();
        }
    });

    let app = Router::new()
        // API routes
        .route("/api/certify", post(api::post_certify))
        .route("/api/verify/{token}", get(api::get_verify_json))
        .route("/api/tokens", get(api::get_tokens_list))
        // HTML pages
        .route("/", get(pages::landing))
        .route("/feed", get(pages::feed))
        .route("/verify/{token}", get(pages::verify_page))
        .route("/v/{token}", get(pages::verify_page))
        // Static files (CSS, etc.)
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("HumanTyped server running on http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server failed");
}
