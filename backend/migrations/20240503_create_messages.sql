CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert test messages
INSERT INTO messages (content, created_at) VALUES
    ('Hello, this is a test message!', CURRENT_TIMESTAMP - INTERVAL '1 hour'),
    ('Testing the messages feature', CURRENT_TIMESTAMP); 