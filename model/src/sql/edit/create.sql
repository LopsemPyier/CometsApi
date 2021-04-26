INSERT INTO edits (project, action_type, author_id, file)
VALUES ($1, $2, $3, $4)
RETURNING id, project, create_at, author_id, file, action_type AS "action_type!: Action";