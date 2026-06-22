CREATE TABLE workspaces (
    id UUID PRIMARY KEY,

    owner_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,
    description TEXT,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
