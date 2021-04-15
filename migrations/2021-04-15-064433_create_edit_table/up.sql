CREATE TYPE ACTION as ENUM (
    'CREATE',
    'DELETE',
    'EDIT'
);

CREATE TABLE edits (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    project uuid REFERENCES projects ON DELETE CASCADE NOT NULL,
    action_type ACTION NOT NULL,
    create_at timestamptz DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    author_id uuid REFERENCES users ON DELETE CASCADE NOT NULL
);