-- Add migration script here
ALTER TABLE sessions ADD COLUMN user_id uuid;

CREATE INDEX session_user_id ON sessions (user_id);