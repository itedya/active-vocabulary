CREATE TABLE example_generation_queue (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (word_id) REFERENCES words (id),
    UNIQUE (word_id)
);