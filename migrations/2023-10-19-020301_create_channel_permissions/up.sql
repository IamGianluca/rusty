-- Your SQL goes here
CREATE TABLE channel_permissions (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  channel_id INT NOT NULL REFERENCES channels(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

