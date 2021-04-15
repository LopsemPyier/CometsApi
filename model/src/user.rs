use super::error::DatabaseError;

use uuid::Uuid;
use sqlx::PgPool;
use crate::project::Project;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub password: String,
	pub email: String
}

impl User {
	pub async fn create(pool: &PgPool, username: String, password: String, email: String) -> Result<User, DatabaseError> {
		let result = sqlx::query_as!(
			User,
			r#"
				INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING *;
			"#,
			username,
			password,
			email
		)
			.fetch_one(pool)
			.await;

		match result {
			Ok(user) => {
				Ok(user)
			}
			Err(err) => {
				eprintln!("{}", err);
				Err(err.into())
			}
		}
	}

	pub async fn get_projects_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, DatabaseError> {
		let projects_row = sqlx::query_as!(
			Project,
			r#"
				SELECT id, name, description FROM projects JOIN users_projects ON users_projects.project_id = projects.id WHERE users_projects.user_id = $1;
			"#,
			user_id
		)
			.fetch_all(pool)
			.await?;

		Ok(projects_row)
	}

	pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, DatabaseError> {
		let users_row = sqlx::query_as!(
			User,
			r#"SELECT * FROM users;"#
		)
			.fetch_all(pool)
			.await?;

		Ok(users_row)
	}

	pub async fn get_by_email(pool: &PgPool, email: &String) -> Result<Option<User>, DatabaseError> {
		let user_row = sqlx::query_as!(
			User,
			r#"SELECT * FROM users WHERE email = $1;"#,
			email
		)
			.fetch_optional(pool)
			.await?;

		Ok(user_row)
	}

	pub async fn get_by_uuid(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, DatabaseError> {
		let user_row = sqlx::query_as!(
			User,
			r#"
				SELECT * FROM users WHERE id = $1
			"#,
			user_id
		)
			.fetch_optional(pool)
			.await?;

		Ok(user_row)
	}
}