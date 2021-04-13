use super::model::user::User;
use uuid::Uuid;
use argonautica::Hasher;
use super::db::Database;
use super::db::Error;


impl Database {
	pub async fn create_user(&self, username: String, password: String, email: String) -> Result<User, Error> {
		let mut hasher = Hasher::default();
		let password_hased = hasher
			.with_password(password)
			.with_secret_key("The secret key")
			.hash()
			.unwrap();
		
		let result = sqlx::query_as!(
			User,
			r#"
				INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING *;
			"#,
			username,
			password_hased,
			email
		)
			.fetch_one(&self.pool)
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
	
	pub async fn get_users(&self) -> Result<Vec<User>, Error> {
		let users_row = sqlx::query_as!(
			User, 
			r#"SELECT * FROM users;"#
		)
			.fetch_all(&self.pool)
			.await?;
	
		Ok(users_row)
	}

	pub async fn get_user_by_uuid(&self, user_id: Uuid) -> Result<User, Error> {
		let user_row = sqlx::query_as!(
			User,
			r#"
				SELECT * FROM users WHERE id = $1
			"#,
			user_id
		)
			.fetch_one(&self.pool)
			.await?;
		
		Ok(user_row)
	}
}

