CREATE TABLE examples (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    example TEXT NOT NULL,
    translation TEXT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES words (id)
);