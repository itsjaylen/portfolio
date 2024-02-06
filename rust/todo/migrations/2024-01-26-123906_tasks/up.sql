-- Your SQL goes here
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed BOOLEAN NOT NULL DEFAULT 0,
    completed_at TIMESTAMP
);