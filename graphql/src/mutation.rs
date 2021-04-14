use super::context::ContextData;

use model::dto::auth::{ LoginDto, RegisterDto };

use service::auth;

use async_graphql::{ Context, FieldResult };

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
}