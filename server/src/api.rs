use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::net::SocketAddr;

use crate::db::{self, CertifyRequest};
use crate::AppState;

#[derive(Serialize)]
pub struct CertifyResponse {
    pub success: bool,
    pub token: String,
    pub verify_url: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wpm_average: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wpm_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn post_certify(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CertifyRequest>,
) -> Result<Json<CertifyResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Rate limit check
    {
        let mut limiter = state.rate_limiter.lock().unwrap();
        if !limiter.check(addr.ip()) {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(ErrorResponse {
                    error: "Rate limited. Max 100 requests per hour.".to_string(),
                }),
            ));
        }
    }

    // Validate token format
    if req.token.len() != 12 || !req.token.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Token must be exactly 12 alphanumeric characters.".to_string(),
            }),
        ));
    }

    // Validate confidence score range
    if req.confidence_score < 0.0 || req.confidence_score > 100.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Confidence score must be between 0 and 100.".to_string(),
            }),
        ));
    }

    let token = req.token.clone();
    match db::certify(&state.db, req).await {
        Ok(()) => Ok(Json(CertifyResponse {
            success: true,
            verify_url: format!("/verify/{}", token),
            token,
        })),
        Err(_) => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Token already exists.".to_string(),
            }),
        )),
    }
}

pub async fn get_tokens_list(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let tokens = db::list_recent(&state.db, 50).await;
    let count = db::count_tokens(&state.db).await;
    Json(serde_json::json!({
        "total": count,
        "tokens": tokens,
    }))
}

pub async fn get_verify_json(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Json<VerifyResponse> {
    match db::verify(&state.db, token).await {
        Some(record) => Json(VerifyResponse {
            valid: true,
            token: Some(record.token),
            wpm_average: Some(record.wpm_average),
            wpm_variance: Some(record.wpm_variance),
            correction_rate: Some(record.correction_rate),
            session_duration_ms: Some(record.session_duration_ms),
            character_count: Some(record.character_count),
            confidence_score: Some(record.confidence_score),
            created_at: Some(record.created_at),
        }),
        None => Json(VerifyResponse {
            valid: false,
            token: None,
            wpm_average: None,
            wpm_variance: None,
            correction_rate: None,
            session_duration_ms: None,
            character_count: None,
            confidence_score: None,
            created_at: None,
        }),
    }
}
