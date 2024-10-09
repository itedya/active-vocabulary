CREATE TABLE examples_of_terms_usage
(
    id                  SERIAL PRIMARY KEY,
    example             TEXT    NOT NULL,
    translation         TEXT    NOT NULL,
    term_id             INTEGER NOT NULL,
    learning_session_id INTEGER DEFAULT NULL,

    FOREIGN KEY (term_id) REFERENCES terms (id) ON DELETE CASCADE,
    FOREIGN KEY (learning_session_id) REFERENCES learning_sessions (id) ON DELETE CASCADE
);