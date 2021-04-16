SELECT id, username, email, password FROM users
    JOIN users_projects
    ON users_projects.user_id = users.id
    WHERE users_projects.project_id = $1;