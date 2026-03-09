use rusqlite::params;
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertifyRequest {
    pub token: String,
    pub wpm_average: f64,
    pub wpm_variance: f64,
    pub correction_rate: f64,
    pub session_duration_ms: i64,
    pub character_count: i64,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRecord {
    pub token: String,
    pub wpm_average: f64,
    pub wpm_variance: f64,
    pub correction_rate: f64,
    pub session_duration_ms: i64,
    pub character_count: i64,
    pub confidence_score: f64,
    pub created_at: String,
}

pub async fn init_db(path: &str) -> Connection {
    let conn = Connection::open(path).await.expect("Failed to open database");

    conn.call(|conn| {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tokens (
                token TEXT PRIMARY KEY,
                wpm_average REAL NOT NULL,
                wpm_variance REAL NOT NULL,
                correction_rate REAL NOT NULL,
                session_duration_ms INTEGER NOT NULL,
                character_count INTEGER NOT NULL,
                confidence_score REAL NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            PRAGMA journal_mode=WAL;",
        )?;
        Ok(())
    })
    .await
    .expect("Failed to initialize database schema");

    conn
}

pub async fn certify(conn: &Connection, req: CertifyRequest) -> Result<(), String> {
    let result = conn
        .call(move |conn| {
            conn.execute(
                "INSERT INTO tokens (token, wpm_average, wpm_variance, correction_rate, session_duration_ms, character_count, confidence_score)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    req.token,
                    req.wpm_average,
                    req.wpm_variance,
                    req.correction_rate,
                    req.session_duration_ms,
                    req.character_count,
                    req.confidence_score,
                ],
            )?;
            Ok(())
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(_) => Err("Token already exists".to_string()),
    }
}

pub async fn list_recent(conn: &Connection, limit: u32) -> Vec<TokenRecord> {
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT token, wpm_average, wpm_variance, correction_rate, session_duration_ms, character_count, confidence_score, created_at
             FROM tokens ORDER BY created_at DESC LIMIT ?1",
        )?;

        let records = stmt
            .query_map(params![limit], |row| {
                Ok(TokenRecord {
                    token: row.get(0)?,
                    wpm_average: row.get(1)?,
                    wpm_variance: row.get(2)?,
                    correction_rate: row.get(3)?,
                    session_duration_ms: row.get(4)?,
                    character_count: row.get(5)?,
                    confidence_score: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(records)
    })
    .await
    .unwrap_or_default()
}

pub async fn count_tokens(conn: &Connection) -> i64 {
    conn.call(|conn| {
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM tokens", [], |row| row.get(0))?;
        Ok(count)
    })
    .await
    .unwrap_or(0)
}

pub async fn verify(conn: &Connection, token: String) -> Option<TokenRecord> {
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT token, wpm_average, wpm_variance, correction_rate, session_duration_ms, character_count, confidence_score, created_at
             FROM tokens WHERE token = ?1",
        )?;

        let record = stmt
            .query_row(params![token], |row| {
                Ok(TokenRecord {
                    token: row.get(0)?,
                    wpm_average: row.get(1)?,
                    wpm_variance: row.get(2)?,
                    correction_rate: row.get(3)?,
                    session_duration_ms: row.get(4)?,
                    character_count: row.get(5)?,
                    confidence_score: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })
            .ok();

        Ok(record)
    })
    .await
    .unwrap_or(None)
}
