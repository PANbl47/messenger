CREATE TABLE accounts (
    account_id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    phone_discoverable BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE linked_identity_methods (
    identity_id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    method_type TEXT NOT NULL,
    method_value TEXT NOT NULL UNIQUE,
    secret_hash TEXT,
    verified_at TIMESTAMP,
    UNIQUE(account_id, method_type)
);

CREATE TABLE recovery_emails (
    account_id TEXT PRIMARY KEY REFERENCES accounts(account_id) ON DELETE CASCADE,
    email_ciphertext TEXT NOT NULL,
    email_hint TEXT NOT NULL,
    linked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE display_name_projection (
    account_id TEXT PRIMARY KEY REFERENCES accounts(account_id) ON DELETE CASCADE,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    normalized_display_name TEXT NOT NULL
);

CREATE INDEX idx_display_name_projection_normalized
    ON display_name_projection(normalized_display_name);
