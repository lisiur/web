-- Add migration script here
CREATE TABLE IF NOT EXISTS authentication(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    grant_type TEXT NOT NULL,
    token TEXT NOT NULL,
    created_at timestamp not null,
    updated_at timestamp not null
)