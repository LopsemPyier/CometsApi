use schema::context::ContextData;

use schema::object::{
	user::UserObject,
	project::ProjectObject
};

use service::{ user, project };

use async_graphql::{Context, FieldResult, Error};
use uuid::Uuid;

pub struct Query;

#[async_graphql::Object]
impl Query{
	pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserObject>> {
		let data = ctx.data::<ContextData>()?;
		user::get_all(&data.db).await
	}

	pub async fn user(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<UserObject> {
		let data = ctx.data::<ContextData>()?;
		user::get_by_uuid(id, &data.db).await
	}

	pub async fn projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<ProjectObject>> {
		let data = ctx.data::<ContextData>()?;
		project::get_all(&data.db).await
	}

	pub async fn project(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<ProjectObject> {
		let data = ctx.data::<ContextData>()?;
		project::get_by_uuid(id, &data.db).await
	}

	pub async fn me(&self, ctx: &Context<'_>) -> FieldResult<UserObject> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(token) = token {
			let data = ctx.data::<ContextData>()?;
			return user::get_by_uuid(token.user_id, &data.db).await;
		}
		Err(Error::new("No token provided"))
	}
}