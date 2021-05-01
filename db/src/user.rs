use uuid::Uuid;

use model::error::DatabaseError;
use model::user::User;
use utils::auth::hash_password;

use super::database::Database;

impl Database {
	pub async fn create_user(&self, username: String, password: String, email: String) -> Result<User, DatabaseError> {
		let password_hashed = hash_password(&password);

		User::create(&self.pool, username, password_hashed, email).await
	}
	
	pub async fn get_users(&self) -> Result<Vec<User>, DatabaseError> {
		User::get_all(&self.pool).await
	}

	pub async fn get_user_by_email(&self, email: &String) -> Result<Option<User>, DatabaseError> {
		User::get_by_email(&self.pool, &email).await
	}

	pub async fn get_user_by_uuid(&self, user_id: Uuid) -> Result<Option<User>, DatabaseError> {
		User::get_by_uuid(&self.pool, user_id).await
	}

	pub async fn update_user(&self, user_id: Uuid, username: &String, email: &String) -> Result<bool, DatabaseError> {
		User::update(&self.pool, user_id, username, email).await
	}

	pub async fn update_password(&self, user_id: Uuid, password: &String) -> Result<bool, DatabaseError> {
		let password_hashed = hash_password(&password);
		User::update_password(&self.pool, user_id, &password_hashed).await
	}
}

