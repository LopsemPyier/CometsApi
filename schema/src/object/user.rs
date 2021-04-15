use uuid::Uuid;
use async_graphql::Context;
use model::user::User;
use crate::context::ContextData;
use super::project::ProjectObject;

#[derive(Debug)]
pub struct UserObject {
	pub id: Uuid,
	pub username: String,
	pub email: String
}

#[async_graphql::Object]
/// A user of the app
impl UserObject {
	/// The user id, store as an UUID
	pub async fn id(&self) -> uuid::Uuid {
		self.id
	}

	/// The username
	pub async fn username(&self) -> &str {
		&self.username
	}

	/// The email
	pub async fn email(&self) -> &str {
		&self.email
	}

	/// The list of the project the user is author in
	pub async fn projects(&self, ctx: &Context<'_>) -> Vec<ProjectObject> {
		let data = ctx.data::<ContextData>().unwrap();
		let projects = User::get_projects_for_user(&data.db.pool, self.id).await.unwrap();
		projects.into_iter().map(ProjectObject::from).collect()
	}
}

impl From<User> for UserObject {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			username: user.username,
			email: user.email
		}
	}
}