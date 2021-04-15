CREATE TABLE users (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	email VARCHAR NOT NULL
);