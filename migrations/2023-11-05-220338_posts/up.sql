-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    category_id INTEGER,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);