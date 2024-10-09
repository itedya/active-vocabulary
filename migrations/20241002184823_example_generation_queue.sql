CREATE TABLE example_generation_queue
(
    id         SERIAL PRIMARY KEY,
    term_id    INTEGER   NOT NULL,
    how_much   INTEGER   NOT NULL CHECK (how_much >= 0),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (term_id) REFERENCES terms (id)
);