-- Add up migration script here
CREATE TYPE file_type as ENUM (
    'IMAGE',
    'TEX',
    'PDF'
    );

CREATE TABLE files
(
    id        uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    project   uuid REFERENCES projects ON DELETE CASCADE  NOT NULL,
    parent    uuid REFERENCES files ON DELETE CASCADE,
    name      varchar                                     NOT NULL,
    file_type file_type                                   NOT NULL,
    extension varchar                                     NOT NULL
);