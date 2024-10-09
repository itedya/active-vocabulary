CREATE TABLE terms
(
    id          SERIAL PRIMARY KEY,
    term        TEXT    NOT NULL,
    translation TEXT    NOT NULL,
    user_id     INTEGER NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at  TIMESTAMP DEFAULT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);