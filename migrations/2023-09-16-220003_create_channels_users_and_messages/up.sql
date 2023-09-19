-- Your SQL goes here
CREATE TABLE channels (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  channel_id INT REFERENCES channels(id),
  user_id INT REFERENCES users(id),
  content TEXT NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL
)
