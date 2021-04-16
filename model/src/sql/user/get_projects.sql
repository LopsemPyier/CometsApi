SELECT id, name, description FROM projects
    JOIN users_projects
    ON users_projects.project_id = projects.id
    WHERE users_projects.user_id = $1;