use async_graphql::{Context, Error, FieldResult};
use uuid::Uuid;

use schema::context::ContextData;
use schema::object::{
	project::ProjectObject,
	user::UserObject,
};
use schema::object::file::FileObject;
use service::{file, project, user};

pub struct Query;

#[async_graphql::Object]
impl Query {
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

	pub async fn files(&self, ctx: &Context<'_>) -> FieldResult<Vec<FileObject>> {
		let data = ctx.data::<ContextData>()?;
		file::get_all(&data.db).await
	}

	pub async fn file(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<FileObject> {
		let data = ctx.data::<ContextData>()?;
		file::get_by_uuid(id, &data.db).await
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