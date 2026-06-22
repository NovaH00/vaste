CREATE TABLE users (
    id UUID PRIMARY KEY,

    display_name TEXT NOT NULL,
    bio TEXT NOT NULL,

    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
