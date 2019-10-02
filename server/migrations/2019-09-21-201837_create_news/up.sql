-- Your SQL goes here
CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    title TEXT,
    content TEXT,
    date DATE DEFAULT NOW(),
    author_id INTEGER
)