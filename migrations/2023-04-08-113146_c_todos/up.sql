CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  user_id TEXT NOT NULL REFERENCES users(id),
  status INTEGER NOT NULL DEFAULT 0
);
