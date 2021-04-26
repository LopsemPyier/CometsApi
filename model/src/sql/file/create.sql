INSERT INTO files (name, project, extension, file_type, parent)
VALUES ($1, $2, $3, $4, $5)
RETURNING *;