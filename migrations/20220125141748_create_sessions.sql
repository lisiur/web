-- Add migration script here
create table if not exists sessions (
    id uuid primary key default uuid_generate_v4(),
    jwt text not null,
    expired_at timestamp not null,
    created_at timestamp not null,
    updated_at timestamp not null
);