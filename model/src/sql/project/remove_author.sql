DELETE
FROM users_projects
WHERE user_id = $1
  and project_id = $2;