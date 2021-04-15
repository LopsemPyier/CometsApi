CREATE TABLE users_projects (
    user_id uuid REFERENCES users ON DELETE CASCADE,
    project_id uuid REFERENCES projects ON DELETE CASCADE,
    PRIMARY KEY (user_id, project_id)
);