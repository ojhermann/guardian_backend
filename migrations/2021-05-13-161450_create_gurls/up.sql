-- Your SQL goes here
CREATE TABLE gurls (
    id SERIAL,
    url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    liked BOOLEAN NOT NULL,
    PRIMARY KEY(url, created_at)
);
