-- Add up migration script here
CREATE TABLE files
(
    id      uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    project uuid REFERENCES projects ON DELETE CASCADE  NOT NULL,
    parent  uuid REFERENCES files ON DELETE CASCADE
);