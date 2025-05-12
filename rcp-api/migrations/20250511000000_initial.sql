-- Add api_tokens table
CREATE TABLE IF NOT EXISTS api_tokens (
    id TEXT PRIMARY KEY,
    token_value TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL
);

-- Add api_sessions table
CREATE TABLE IF NOT EXISTS api_sessions (
    id TEXT PRIMARY KEY,
    token_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_active TEXT NOT NULL,
    FOREIGN KEY (token_id) REFERENCES api_tokens(id) ON DELETE CASCADE
);

-- Add audit_logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    timestamp TEXT NOT NULL,
    user_id TEXT,
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    details TEXT
);
