SELECT id, project, create_at, author_id, file, action_type AS "action_type!: Action"
FROM edits
WHERE project = $1;