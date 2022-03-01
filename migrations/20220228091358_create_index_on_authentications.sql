-- Add migration script here
CREATE INDEX user_grant_type ON authentications (user_id, grant_type);
