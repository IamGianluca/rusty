-- Your SQL goes here
CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  password TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

