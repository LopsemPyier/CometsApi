-- Add up migration script here
CREATE TABLE projects
(
    id          uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    name        VARCHAR                                     NOT NULL,
    description TEXT                                        NOT NULL
);