INSERT INTO edits (project, action_type, author_id) VALUES ($1, $2, $3)
    RETURNING id, project, create_at, author_id, action_type AS "action_type!: Action";