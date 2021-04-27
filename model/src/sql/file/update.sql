UPDATE files
SET (name, parent) = ($2, $3)
WHERE id = $1
RETURNING id, name, project, extension, parent, file_type AS "file_type!: FileType";