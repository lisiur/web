-- Add migration script here
ALTER TABLE users DROP COLUMN salt;
ALTER TABLE authentications ADD salt TEXT DEFAULT 'salt';