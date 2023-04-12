CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    telegram_id TEXT NOT NULL UNIQUE
);
