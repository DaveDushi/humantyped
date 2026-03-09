CREATE TABLE IF NOT EXISTS tokens (
    token TEXT PRIMARY KEY,
    wpm_average DOUBLE PRECISION NOT NULL,
    wpm_variance DOUBLE PRECISION NOT NULL,
    correction_rate DOUBLE PRECISION NOT NULL,
    session_duration_ms BIGINT NOT NULL,
    character_count BIGINT NOT NULL,
    confidence_score DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
