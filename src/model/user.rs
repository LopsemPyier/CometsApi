use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub password: String,
	pub email: String
}

#[async_graphql::Object]
impl User {
	pub async fn id(&self) -> async_graphql::ID {
		self.id.into()
	}

	pub async fn username(&self) -> &str {
		&self.username
	}
	
	pub async fn email(&self) -> &str {
		&self.email
	} 
}