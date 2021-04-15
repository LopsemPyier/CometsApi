CREATE TABLE users (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	username VARCHAR,
	password VARCHAR,
	email VARCHAR
);