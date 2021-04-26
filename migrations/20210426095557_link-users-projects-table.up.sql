-- Add up migration script here
CREATE TABLE users_projects
(
    user_id    uuid REFERENCES users ON DELETE CASCADE    NOT NULL,
    project_id uuid REFERENCES projects ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (user_id, project_id)
);