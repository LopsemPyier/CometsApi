use schema::context::ContextData;

use schema::dto::auth::{ LoginDto, RegisterDto };
use schema::dto::project::ProjectDto;

use service::{ auth, project };

use async_graphql::{Context, FieldResult, Error};
use schema::object::project::ProjectObject;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation{
	pub async fn register(&self, ctx: &Context<'_>, input: RegisterDto) -> FieldResult<String> {
		let data = ctx.data::<ContextData>()?;

		auth::register(input, &data.db).await
	}

	pub async fn login(&self, ctx: &Context<'_>, input: LoginDto) -> FieldResult<String> {
		let data = ctx.data::<ContextData>()?;

		auth::login(input, &data.db).await
	}

	pub async fn create_project(&self, ctx: &Context<'_>, input: ProjectDto) -> FieldResult<ProjectObject> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(token) = token {
			let data = ctx.data::<ContextData>()?;
			return project::create(&data.db, token.user_id, input.name, input.description).await;
		}
		Err(Error::new("No token provided"))
	}
}