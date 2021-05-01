UPDATE users
SET username = $2,
    email    = $3
WHERE id = $1;