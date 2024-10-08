CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL unique,
    password TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);