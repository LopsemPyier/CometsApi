use async_graphql::{Context, Error, FieldResult};
use uuid::Uuid;

use schema::context::ContextData;
use schema::dto::auth::{LoginDto, RegisterDto, UpdateUserDto};
use schema::dto::file::{FileDto, UpdateFileDto};
use schema::dto::project::{ProjectDto, UpdateProjectDto};
use schema::object::file::FileObject;
use schema::object::project::ProjectObject;
use service::{auth, file, project};

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
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

	pub async fn add_author(&self, ctx: &Context<'_>, project_id: Uuid, author_id: Uuid) -> FieldResult<bool> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return project::add_author(&data.db, project_id, author_id).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn remove_author(&self, ctx: &Context<'_>, project_id: Uuid, author_id: Uuid) -> FieldResult<bool> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return project::remove_author(&data.db, project_id, author_id).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn create_file(&self, ctx: &Context<'_>, input: FileDto) -> FieldResult<FileObject> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return file::create(&data.db, input.name, input.extension, input.project_id, input.parent_id, input.folder).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn delete_file(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<bool> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return file::delete(&data.db, id).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<bool> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return project::delete(&data.db, id).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn update_project(&self, ctx: &Context<'_>, input: UpdateProjectDto) -> FieldResult<ProjectObject> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return project::update(&data.db, input.id, input.name, input.description).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn update_password(&self, ctx: &Context<'_>, old: String, new: String) -> FieldResult<bool> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(token) = token {
			let data = ctx.data::<ContextData>()?;
			return auth::update_password(&data.db, token.user_id, old, new).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserDto) -> FieldResult<String> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(token) = token {
			let data = ctx.data::<ContextData>()?;
			return auth::update(&data.db, token.user_id, input.username, input.email).await;
		}
		Err(Error::new("No token provided"))
	}

	pub async fn update_file(&self, ctx: &Context<'_>, id: Uuid, input: UpdateFileDto) -> FieldResult<FileObject> {
		let token = ctx.data_opt::<utils::auth::ContextToken>();
		if let Some(_token) = token {
			let data = ctx.data::<ContextData>()?;
			return file::update(&data.db, id, input.name, input.parent_id).await;
		}
		Err(Error::new("No token provided"))
	}
}